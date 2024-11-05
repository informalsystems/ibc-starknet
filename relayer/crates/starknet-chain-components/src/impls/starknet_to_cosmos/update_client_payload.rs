use core::time::Duration;

use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_chain_components::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_chain_components::traits::types::client_state::HasClientStateType;
use hermes_chain_components::traits::types::create_client::HasCreateClientPayloadOptionsType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::update_client::HasUpdateClientPayloadType;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer::config::types::TrustThreshold;

pub struct BuildUpdateCometClientPayload;

impl<Chain, Counterparty> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildUpdateCometClientPayload
where
    Chain: CanBuildCreateClientPayload<Counterparty>
        + HasCreateClientPayloadOptionsType<Counterparty, CreateClientPayloadOptions = Settings>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = Chain::CreateClientPayload>
        + HasClientStateType<Counterparty>
        + HasHeightType
        + HasErrorType,
{
    async fn build_update_client_payload(
        chain: &Chain,
        _trusted_height: &Chain::Height,
        _target_height: &Chain::Height,
        _client_state: Chain::ClientState,
    ) -> Result<Chain::UpdateClientPayload, Chain::Error> {
        let create_client_settings = Settings {
            max_clock_drift: Duration::from_secs(40),
            trusting_period: Some(Duration::from_secs(60 * 60)),
            trust_threshold: TrustThreshold::ONE_THIRD,
        };

        let payload = chain
            .build_create_client_payload(&create_client_settings)
            .await?;

        Ok(payload)
    }
}
