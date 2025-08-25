use core::marker::PhantomData;

use attestator::{AttestatorClient, Ed25519};
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryClientStateWithLatestHeight, HasClientIdType, HasClientStateType,
    HasCreateClientMessageOptionsType, HasMessageType, HasUpdateClientPayloadType,
    UpdateClientMessageBuilder, UpdateClientMessageBuilderComponent,
};
use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::{LevelDebug, LevelWarn};
use hermes_cosmos_core::chain_components::types::CosmosUpdateClientPayload;
use hermes_prelude::*;
use ibc::clients::tendermint::types::Header;
use ibc_proto::ibc::lightclients::tendermint::v1::Header as RawHeader;
use ibc_proto::Protobuf;
use num_bigint::BigUint;
use rand::seq::SliceRandom;
use starknet::core::types::{ByteArray, Felt, U256};
use starknet::macros::selector;
use tendermint::block::CommitSig;
use tendermint::vote::{SignedVote, ValidatorIndex, Vote};

use crate::impls::{StarknetAddress, StarknetMessage};
use crate::traits::{CanQueryContractAddress, HasEd25519AttestatorAddresses};
use crate::types::{ClientId, ClientMessage, CometClientState};

pub struct BuildUpdateCometClientMessage;

#[cgp_provider(UpdateClientMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateCometClientMessage
where
    Chain: HasCreateClientMessageOptionsType<Counterparty>
        + HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + HasClientIdType<Counterparty, ClientId = ClientId>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanQueryClientStateWithLatestHeight<Counterparty>
        + CanLog<LevelWarn>
        + CanLog<LevelDebug>
        + HasEd25519AttestatorAddresses
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasClientStateType<Chain, ClientState = CometClientState>
        + HasUpdateClientPayloadType<Chain, UpdateClientPayload = CosmosUpdateClientPayload>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![Product![U256, U256, U256, Vec<u8>], Vec<Felt>, U256, U256]>
        + CanEncode<ViaCairo, Product![Vec<Felt>, U256, U256]>
        + CanEncode<ViaCairo, Vec<Product![Felt, Felt, Felt]>>
        + CanEncode<ViaCairo, Vec<Vec<Felt>>>
        + CanEncode<ViaCairo, Product![ClientMessage, Vec<Felt>]>
        + CanEncode<ViaCairo, ByteArray>
        + CanEncode<ViaCairo, Product![ClientId, Vec<Felt>]>,
{
    async fn build_update_client_message(
        chain: &Chain,
        client_id: &ClientId,
        counterparty_payload: CosmosUpdateClientPayload,
    ) -> Result<Vec<Chain::Message>, Chain::Error> {
        let mut messages = Vec::with_capacity(counterparty_payload.headers.len());

        for header in counterparty_payload.headers {
            let encoding = chain.encoding();

            let contract_address = chain.query_contract_address(PhantomData).await?;

            // We are not passing the Cairo serialization of the Client Header here.
            // As it has a lot of hash fields as `Vec<u8>`. In the Cairo serialization,
            // they are be encoded as `Array<felt252>` wasting a lot of encoding data space.
            //
            // So, we encode the Header as Protobuf bytes and then encode those bytes as
            // Cairo `ByteArray` which has more succinct `Vec<u8>` representation.

            let protobuf_bytes = Protobuf::<RawHeader>::encode_vec(header.clone());

            let protobuf_byte_array: ByteArray = protobuf_bytes.into();

            let raw_header = encoding
                .encode(&protobuf_byte_array)
                .map_err(Chain::raise_error)?;

            let ed25519_attestator_addresses = chain
                .ed25519_attestator_addresses()
                .as_ref()
                .ok_or("No Ed25519 attestators")
                .map_err(Chain::raise_error)?;

            let client_state = chain
                .query_client_state_with_latest_height(PhantomData, client_id)
                .await?;

            let attestator_keys = client_state.attestator_keys;

            if attestator_keys.len() != ed25519_attestator_addresses.len() {
                return Err(Chain::raise_error(
                    "Attestator keys and addresses length mismatch",
                ));
            }

            let attestator_quorum_percentage = client_state.attestator_quorum_percentage;

            let signature_hints = comet_signature_hints(
                chain,
                &header,
                encoding,
                ed25519_attestator_addresses,
                attestator_quorum_percentage,
            )
            .await;

            let serialized_signature_hints = encoding
                .encode(&signature_hints)
                .map_err(Chain::raise_error)?;

            let client_message_with_hints = product![
                ClientMessage::Update(raw_header),
                serialized_signature_hints
            ];

            let client_message_felts = encoding
                .encode(&client_message_with_hints)
                .map_err(Chain::raise_error)?;

            let calldata = encoding
                .encode(&product![client_id.clone(), client_message_felts])
                .map_err(Chain::raise_error)?;

            let message =
                StarknetMessage::new(*contract_address, selector!("update_client"), calldata);

            messages.push(message);
        }

        Ok(messages)
    }
}

pub async fn comet_signature_hints<Chain, Encoding>(
    chain: &Chain,
    header: &Header,
    encoding: &Encoding,
    attestator_addresses: &[String],
    attestator_quorum_percentage: usize,
) -> Vec<Vec<Felt>>
where
    Chain: CanLog<LevelWarn> + CanLog<LevelDebug>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![Product![U256, U256, U256, Vec<u8>], Vec<Felt>, U256, U256]>
        + CanEncode<ViaCairo, Product![Vec<Felt>, U256, U256]>
        + CanEncode<ViaCairo, Vec<Product![Felt, Felt, Felt]>>
        + CanEncode<ViaCairo, Vec<Vec<Felt>>>,
{
    let signed_header = &header.signed_header;
    let validators: std::collections::HashMap<_, _> = header
        .validator_set
        .validators()
        .iter()
        .map(|v| (v.address, v.pub_key))
        .collect();

    let mut hints: Vec<Vec<Felt>> = Vec::new();

    for value in signed_header
        .commit
        .signatures
        .iter()
        .enumerate()
        .map(|(idx, signature)| {
            let validator_index = ValidatorIndex::try_from(idx).unwrap();

            let CommitSig::BlockIdFlagCommit {
                validator_address,
                timestamp,
                signature,
            } = signature
            else {
                return None;
            };

            let vote = Vote {
                vote_type: tendermint::vote::Type::Precommit,
                height: signed_header.commit.height,
                round: signed_header.commit.round,
                block_id: Some(signed_header.commit.block_id),
                timestamp: Some(*timestamp),
                validator_address: *validator_address,
                validator_index,
                signature: signature.clone(),
                extension: Default::default(),
                extension_signature: None,
            };

            let signed_vote = SignedVote::from_vote(vote, signed_header.header.chain_id.clone())?;

            let msg = signed_vote.sign_bytes();
            let signature: [u8; 64] = signed_vote.signature().as_bytes().try_into().ok()?;
            let validator_id = signed_vote.validator_id();

            let validator_public_key = validators.get(&validator_id)?;

            let tendermint::PublicKey::Ed25519(ed25519_public_key) = validator_public_key else {
                // If the public key is not Ed25519, we can return None or handle accordingly.
                return None;
            };

            let ed25519_public_key: [u8; 32] = ed25519_public_key.as_bytes().try_into().ok()?;

            Some((msg, signature, ed25519_public_key))
        })
    {
        if let Some((msg, signature, public_key)) = value {
            hints.push(
                compute_attestator_hints(
                    chain,
                    encoding,
                    attestator_addresses,
                    attestator_quorum_percentage,
                    &msg,
                    &signature,
                    &public_key,
                )
                .await,
            );
        } else {
            // only return hints for the valid signatures
            hints.push(vec![]);
        }
    }

    hints
}

pub fn compute_garaga_hints<Encoding>(
    encoding: &Encoding,
    msg: &[u8],
    signature: &[u8; 64],
    public_key: &[u8; 32],
) -> Vec<Felt>
where
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![Product![U256, U256, U256, Vec<u8>], Vec<Felt>, U256, U256]>
        + CanEncode<ViaCairo, Product![Vec<Felt>, U256, U256]>,
{
    let ry_twisted = BigUint::from_bytes_le(&signature[0..32]);
    let s = BigUint::from_bytes_le(&signature[32..64]);
    let py_twisted = BigUint::from_bytes_le(public_key);

    let hint = garaga::calldata::signatures::eddsa_calldata_builder(
        ry_twisted,
        s,
        py_twisted,
        msg.to_vec(),
    )
    .unwrap();

    let felt_hint = hint
        .into_iter()
        .map(|x| Felt::from_hex(&format!("{x:x}")).unwrap())
        .collect::<Vec<Felt>>();

    let product![
        product![p_ry_twisted, p_s, p_py_twisted, p_msg],
        p_msm_hint,
        p_sqrt_rx_hint,
        p_sqrt_px_hint
    ]: Product![Product![U256, U256, U256, Vec<u8>], Vec<Felt>, U256, U256] =
        encoding.decode(&felt_hint).unwrap();

    assert_eq!(p_msg, msg);

    encoding
        .encode(&product![p_msm_hint, p_sqrt_rx_hint, p_sqrt_px_hint])
        .unwrap()
}

pub async fn compute_attestator_hints<Chain, Encoding>(
    chain: &Chain,
    encoding: &Encoding,
    attestator_addresses: &[String],
    attestator_quorum_percentage: usize,
    msg: &[u8],
    signature: &[u8; 64],
    public_key: &[u8; 32],
) -> Vec<Felt>
where
    Chain: CanLog<LevelWarn> + CanLog<LevelDebug>,
    Encoding:
        HasEncodedType<Encoded = Vec<Felt>> + CanEncode<ViaCairo, Vec<Product![Felt, Felt, Felt]>>,
{
    let mut signatures = Vec::new();

    let mut attestator_addresses = attestator_addresses.to_vec();
    // shuffle attestator_addresses
    attestator_addresses.shuffle(&mut rand::thread_rng());

    let mut attestation_count = 0;

    for client in attestator_addresses
        .iter()
        .map(|addr| AttestatorClient(addr.as_str()))
    {
        // Error calls will be ignored: `.ok()?`
        // This allows attestator network to be fault-tolerant.

        let (public_key, r, s) = match client.get_attestation(&[Ed25519 {
            message: msg.to_vec(),
            signature: *signature,
            public_key: *public_key,
        }]) {
            Ok((public_key, signatures)) => {
                if signatures.len() != 1 {
                    chain
                        .log(
                            &format!("Unexpected number of signatures: {}", signatures.len()),
                            &LevelWarn,
                        )
                        .await;
                    continue;
                } else {
                    let (r, s) = signatures[0];
                    (public_key, r, s)
                }
            }
            Err(err) => {
                chain
                    .log(&format!("Failed to get attestation: {err}"), &LevelWarn)
                    .await;
                continue;
            }
        };

        signatures.push(product![public_key, r, s]);
        if attestation_count * 100 >= attestator_quorum_percentage * attestator_addresses.len() {
            chain
                .log(
                    &format!("Reached attestator quorum with {attestation_count} attestators"),
                    &LevelDebug,
                )
                .await;
            break;
        }
        attestation_count += 1;
    }

    encoding.encode(&signatures).unwrap()
}
