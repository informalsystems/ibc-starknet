use core::time::Duration;

use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_chain_components::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_chain_components::traits::types::client_state::HasClientStateType;
use hermes_chain_components::traits::types::create_client::{
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientPayload;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer::config::types::TrustThreshold;
use ibc_relayer_types::core::ics02_client::height::Height as CosmosHeight;

use crate::types::cosmos::height::Height;
use crate::types::cosmos::update::CometUpdateHeader;

pub struct BuildUpdateCometClientPayload;

/*
   Stub implementation of UpdateClient payload builder from Cosmos to Starknet.
   We basically build a `CosmosCreateClientPayload`, and then use the state root
   of the consensus state to build a dummy update header.

   TODO: refactor Cosmos UpdateClient header so that they are more easily
   be converted into the Cairo encoding for the Starknet Comet contract.
*/
impl<Chain, Counterparty> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildUpdateCometClientPayload
where
    Chain: CanBuildCreateClientPayload<Counterparty>
        + HasCreateClientPayloadOptionsType<Counterparty, CreateClientPayloadOptions = Settings>
        + HasCreateClientPayloadType<Counterparty, CreateClientPayload = CosmosCreateClientPayload>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = CometUpdateHeader>
        + HasClientStateType<Counterparty>
        + HasHeightType<Height = CosmosHeight>
        + HasErrorType,
{
    async fn build_update_client_payload(
        chain: &Chain,
        trusted_height: &CosmosHeight,
        // Ignore the target height for the dummy implementation
        _target_height: &CosmosHeight,
        _client_state: Chain::ClientState,
    ) -> Result<CometUpdateHeader, Chain::Error> {
        let create_client_settings = Settings {
            max_clock_drift: Duration::from_secs(40),
            trusting_period: Some(Duration::from_secs(60 * 60)),
            trust_threshold: TrustThreshold::ONE_THIRD,
        };

        let payload = chain
            .build_create_client_payload(&create_client_settings)
            .await?;

        let height_2 = Height {
            revision_number: payload.client_state.latest_height().revision_number(),
            revision_height: payload.client_state.latest_height().revision_height(),
        };

        let update_header = CometUpdateHeader {
            trusted_height: Height {
                revision_number: trusted_height.revision_number(),
                revision_height: trusted_height.revision_height(),
            },
            target_height: height_2,
            time: payload.consensus_state.timestamp.unix_timestamp() as u64,
            root: payload.consensus_state.root.into_vec(),
        };

        Ok(update_header)
    }
}
