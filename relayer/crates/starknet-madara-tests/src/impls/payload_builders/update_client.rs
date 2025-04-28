use cgp::prelude::*;
use hermes_core::chain_components::traits::{
    CanQueryBlock, HasClientStateType, HasHeightType, HasUpdateClientPayloadType,
    UpdateClientPayloadBuilder, UpdateClientPayloadBuilderComponent,
};
use hermes_core::encoding_components::traits::{CanEncode, HasDefaultEncoding};
use hermes_core::encoding_components::types::AsBytes;
use hermes_cosmos_core::chain_components::types::Secp256k1KeyPair;
use hermes_cosmos_core::protobuf_encoding_components::types::strategy::ViaProtobuf;
use hermes_starknet_chain_components::traits::client::HasStarknetClient;
use hermes_starknet_chain_components::traits::proof_signer::HasStarknetProofSigner;
use hermes_starknet_chain_components::types::consensus_state::StarknetConsensusState;
use hermes_starknet_chain_components::types::payloads::client::StarknetUpdateClientPayload;
use hermes_starknet_chain_components::types::status::StarknetChainStatus;
use ibc::core::client::types::Height;
use ibc::primitives::Timestamp;
use ibc_client_starknet_types::header::StarknetHeader;
use starknet_v13::providers::ProviderError;

#[cgp_new_provider(UpdateClientPayloadBuilderComponent)]
impl<Chain, Counterparty, Encoding> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildStarknetUpdateClientPayload
where
    Chain: HasHeightType<Height = u64>
        + HasClientStateType<Counterparty>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = StarknetUpdateClientPayload>
        + CanQueryBlock<Block = StarknetChainStatus>
        + HasStarknetClient
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
        let block = chain.query_block(target_height).await?;

        let root = Vec::from(block.block_hash.to_bytes_be());

        let consensus_state = StarknetConsensusState {
            root: root.into(),
            time: Timestamp::from_nanoseconds(
                u64::try_from(block.time.unix_timestamp_nanos()).unwrap(),
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
