use cgp::extra::runtime::HasRuntime;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_comet_light_client_context::contexts::light_client::CometLightClient;
use hermes_core::chain_components::traits::{
    HasChainId, HasClientStateType, HasEvidenceType, HasUpdateClientEvent, MisbehaviourChecker,
    MisbehaviourCheckerComponent,
};
use hermes_core::encoding_components::traits::{CanDecode, HasDefaultEncoding, HasEncodedType};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelWarn;
use hermes_core::runtime_components::traits::CanSleep;
use hermes_cosmos_core::chain_components::traits::HasRpcClient;
use hermes_cosmos_core::chain_components::types::TendermintClientState;
use hermes_cosmos_core::comet_light_client_components::traits::{
    CanDetectMisbehaviour, CanFetchLightBlock,
};
use hermes_cosmos_core::tendermint_rpc::Client;
use hermes_error::HermesError;
use hermes_prelude::*;
use ibc_client_tendermint::types::error::TendermintClientError;
use ibc_client_tendermint::types::proto::v1::Misbehaviour;
use ibc_proto::ibc::lightclients::tendermint::v1::Header;
use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;
use starknet::core::types::{ByteArray, Felt};
use tendermint::block::Height as TendermintHeight;
use tendermint::error::Error as TendermintError;
use tendermint::validator::Set;
use tendermint_light_client_verifier::types::{LightBlock, ValidatorSet};
use tendermint_rpc::{Error as TendermintRpcError, Paging};

use crate::impls::{CosmosStarknetMisbehaviour, StarknetUpdateClientEvent};
use crate::types::ClientMessage;

#[cgp_new_provider(MisbehaviourCheckerComponent)]
impl<Chain, Counterparty, Encoding> MisbehaviourChecker<Chain, Counterparty>
    for CheckCosmosMisbehaviourFromStarknet
where
    Chain: HasClientStateType<Counterparty>
        + HasRuntime
        + HasChainId
        + HasRpcClient
        + CanLog<LevelWarn>
        + CanRaiseAsyncError<Counterparty::Error>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<DecodeError>
        + CanRaiseAsyncError<EncodeError>
        + CanRaiseAsyncError<HermesError>
        + CanRaiseAsyncError<TendermintError>
        + CanRaiseAsyncError<TendermintClientError>
        + CanRaiseAsyncError<TendermintRpcError>
        + CanRaiseAsyncError<&'static str>,
    Counterparty: HasUpdateClientEvent<UpdateClientEvent = StarknetUpdateClientEvent>
        + HasChainId
        + HasEvidenceType<Evidence = Any>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>
        + HasAsyncErrorType,
    TendermintClientState: From<Chain::ClientState>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![ClientMessage, Vec<Felt>]>
        + CanDecode<ViaCairo, ByteArray>
        + HasAsyncErrorType,
    Chain::Runtime: CanSleep,
{
    async fn check_misbehaviour(
        chain: &Chain,
        update_client_event: &Counterparty::UpdateClientEvent,
        client_state: &Chain::ClientState,
    ) -> Result<Option<Counterparty::Evidence>, Chain::Error> {
        let encoding = Counterparty::default_encoding();

        let product![client_message, signature_hints,]: Product![ClientMessage, Vec<Felt>] =
            encoding
                .decode(&update_client_event.header)
                .map_err(Chain::raise_error)?;

        let raw_header: Vec<u8> = if let ClientMessage::Update(header) = client_message {
            let byte_array_header: ByteArray =
                encoding.decode(&header).map_err(Chain::raise_error)?;
            byte_array_header.into()
        } else {
            return Err(Chain::raise_error(
                "failed to extract raw header from event",
            ));
        };

        let header: Header = Message::decode(&*raw_header).map_err(Chain::raise_error)?;

        let signed_header = header.clone().signed_header.ok_or_else(|| {
            Chain::raise_error("`signed_header` missing from `Header` in Update Client event")
        })?;

        let trusted_height = header.trusted_height.ok_or_else(|| {
            Chain::raise_error("`trusted_height` missing from `Header` in Update Client event")
        })?;

        let tm_trusted_height = TendermintHeight::try_from(trusted_height.revision_height)
            .map_err(Chain::raise_error)?;

        let tm_client_state = TendermintClientState::from(client_state.clone());

        let rpc_client = chain.rpc_client().clone();

        let status = rpc_client.status().await.map_err(Chain::raise_error)?;

        let current_time = status.sync_info.latest_block_time;
        let peer_id = status.node_info.id;

        let light_client_options = tm_client_state
            .as_light_client_options()
            .map_err(Chain::raise_error)?;

        let light_client = CometLightClient::new(
            chain.chain_id().to_string(),
            current_time,
            peer_id,
            rpc_client.clone(),
            light_client_options,
        );

        let trusted_validator_set: Set = header
            .clone()
            .trusted_validators
            .ok_or_else(|| {
                Chain::raise_error(
                    "`trusted_validators` missing from `Header` in Update Client event",
                )
            })?
            .try_into()
            .map_err(Chain::raise_error)?;

        let signed_header_header = signed_header.clone().header.ok_or_else(|| {
            Chain::raise_error("`header` missing from `SignedHeader` in Update Client event")
        })?;

        let next_validator_height = (signed_header_header.height + 1) as u32;

        let next_validator_proposer_address = signed_header_header
            .clone()
            .proposer_address
            .try_into()
            .map_err(Chain::raise_error)?;

        let next_validators = rpc_client
            .validators(next_validator_height, Paging::All)
            .await
            .map_err(Chain::raise_error)?
            .validators;

        let next_validator_set =
            ValidatorSet::with_proposer(next_validators, next_validator_proposer_address)
                .map_err(Chain::raise_error)?;

        let target_block: LightBlock = LightBlock {
            signed_header: signed_header.try_into().map_err(Chain::raise_error)?,
            validators: trusted_validator_set.clone(),
            next_validators: next_validator_set,
            provider: peer_id,
        };

        let trusted_block = light_client
            .fetch_light_block(&tm_trusted_height.increment())
            .await
            .map_err(Chain::raise_error)?;

        // Required to avoid bad witness error
        chain
            .runtime()
            .sleep(core::time::Duration::from_secs(1))
            .await;

        let maybe_divergence = light_client
            .detect(&target_block, &trusted_block)
            .await
            .map_err(Chain::raise_error)?;

        if let Some(divergence) = maybe_divergence {
            chain
                .log(
                    "Found divergence while checking for misbehaviour",
                    &LevelWarn,
                )
                .await;

            let supporting = divergence
                .evidence
                .witness_trace
                .into_vec()
                .into_iter()
                .filter(|lb| {
                    lb.height() != target_block.height() && lb.height() != tm_trusted_height
                })
                .collect::<Vec<LightBlock>>();

            let trusted_validator_set = light_client
                .fetch_light_block(&tm_trusted_height.increment())
                .await
                .map_err(Chain::raise_error)?
                .validators;

            let mut supporting_headers = Vec::with_capacity(supporting.len());

            let mut current_trusted_height = trusted_height;
            let mut current_trusted_validators = trusted_validator_set.clone();

            for support in supporting {
                let header = Header {
                    signed_header: Some(support.signed_header.clone().into()),
                    validator_set: Some(support.validators.into()),
                    trusted_height: Some(current_trusted_height),
                    trusted_validators: Some(current_trusted_validators.into()),
                };

                // This header is now considered to be the currently trusted header
                current_trusted_height = header.trusted_height.ok_or_else(|| {
                    Chain::raise_error("`trusted_height` missing from support `Header`")
                })?;

                let next_height = TendermintHeight::try_from(
                    header
                        .trusted_height
                        .ok_or_else(|| {
                            Chain::raise_error("`trusted_height` missing from support `Header`")
                        })?
                        .revision_height,
                )
                .map_err(Chain::raise_error)?
                .increment();

                // Therefore we can now trust the next validator set, see NOTE above.
                current_trusted_validators = light_client
                    .fetch_light_block(&next_height)
                    .await
                    .map_err(Chain::raise_error)?
                    .validators;

                supporting_headers.push(header);
            }

            // a) Set the trusted height of the target header to the height of the previous
            // supporting header if any, or to the initial trusting height otherwise.
            //
            // b) Set the trusted validators of the target header to the validators of the successor to
            // the last supporting header if any, or to the initial trusted validators otherwise.
            let (latest_trusted_height, latest_trusted_validator_set) = match supporting_headers
                .last()
            {
                Some(prev_header) => {
                    let prev_height = TendermintHeight::try_from(
                        prev_header
                            .trusted_height
                            .ok_or_else(|| {
                                Chain::raise_error(
                                    "`trusted_height` missing from previous `Header`",
                                )
                            })?
                            .revision_height
                            + 1,
                    )
                    .map_err(Chain::raise_error)?;
                    let prev_succ = light_client
                        .fetch_light_block(&prev_height)
                        .await
                        .map_err(Chain::raise_error)?;
                    (
                        prev_header.trusted_height.ok_or_else(|| {
                            Chain::raise_error("`trusted_height` missing from previous `Header`")
                        })?,
                        prev_succ.validators,
                    )
                }
                None => (trusted_height, trusted_validator_set),
            };

            let evidence: Misbehaviour = CosmosStarknetMisbehaviour {
                client_id: update_client_event.client_id.clone(),
                evidence_1: header,
                evidence_2: Header {
                    signed_header: Some(divergence.challenging_block.signed_header.into()),
                    validator_set: Some(divergence.challenging_block.validators.into()),
                    trusted_height: Some(latest_trusted_height),
                    trusted_validators: Some(latest_trusted_validator_set.into()),
                },
            }
            .into();

            let evidence_any = Any::from_msg(&evidence).map_err(Chain::raise_error)?;

            return Ok(Some(evidence_any));
        }
        chain
            .log(
                "No divergence found while checking for misbehaviour",
                &LevelWarn,
            )
            .await;
        Ok(None)
    }
}
