use cgp::prelude::*;
use hermes_chain_components::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::types::AsBytes;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc::core::client::types::Height;
use ibc::primitives::Timestamp;
use ibc_client_starknet_types::header::StarknetHeader;
use starknet::providers::ProviderError;

use crate::traits::proof_signer::HasStarknetProofSigner;
use crate::traits::provider::HasStarknetProvider;
use crate::traits::queries::status_at_height::CanQueryChainStatusAtHeight;
use crate::types::consensus_state::StarknetConsensusState;
use crate::types::payloads::client::StarknetUpdateClientPayload;
use crate::types::status::StarknetChainStatus;

pub struct BuildStarknetUpdateClientPayload;

#[cgp_provider(UpdateClientPayloadBuilderComponent)]
impl<Chain, Counterparty, Encoding> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildStarknetUpdateClientPayload
where
    Chain: HasHeightType<Height = u64>
        + HasClientStateType<Counterparty>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = StarknetUpdateClientPayload>
        + CanQueryChainStatusAtHeight<ChainStatus = StarknetChainStatus>
        + HasStarknetProvider
        + CanRaiseAsyncError<&'static str>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>
        + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<ProviderError>
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: Async + CanEncode<ViaProtobuf, StarknetHeader, Encoded = Vec<u8>>,
{
    async fn build_update_client_payload(
        chain: &Chain,
        _trusted_height: &u64,
        target_height: &u64,
        _client_state: Chain::ClientState,
    ) -> Result<Chain::UpdateClientPayload, Chain::Error> {
        let chain_status = chain.query_chain_status_at_height(target_height).await?;

        let root = Vec::from(chain_status.block_hash.to_bytes_be());

        let consensus_state = StarknetConsensusState {
            root: root.into(),
            time: Timestamp::from_nanoseconds(
                u64::try_from(chain_status.time.unix_timestamp()).unwrap() * 1_000_000_000,
            ),
        };

        let height = Height::new(0, *target_height).unwrap();

        let header = StarknetHeader {
            height,
            consensus_state,
        };

        let encoded_header = Chain::default_encoding()
            .encode(&header)
            .map_err(Chain::raise_error)?;

        let signature = chain
            .proof_signer()
            .sign(&encoded_header)
            .map_err(Chain::raise_error)?;

        Ok(StarknetUpdateClientPayload { header, signature })
    }
}
