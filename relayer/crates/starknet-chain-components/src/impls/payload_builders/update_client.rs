use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc::primitives::Timestamp;

use crate::types::consensus_state::StarknetConsensusState;
use crate::types::payloads::client::StarknetUpdateClientPayload;
use crate::types::status::StarknetChainStatus;

pub struct BuildStarknetUpdateClientPayload;

impl<Chain, Counterparty> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildStarknetUpdateClientPayload
where
    Chain: HasHeightType
        + HasClientStateType<Counterparty>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = StarknetUpdateClientPayload>
        + CanQueryChainStatus<ChainStatus = StarknetChainStatus>,
{
    async fn build_update_client_payload(
        chain: &Chain,
        _trusted_height: &Chain::Height,
        _target_height: &Chain::Height,
        _client_state: Chain::ClientState,
    ) -> Result<Chain::UpdateClientPayload, Chain::Error> {
        let status = chain.query_chain_status().await?;

        let root = Vec::from(status.block_hash.to_bytes_be());

        let consensus_state = StarknetConsensusState {
            root: root.into(),
            time: Timestamp::now(),
        };

        Ok(StarknetUpdateClientPayload { consensus_state })
    }
}
