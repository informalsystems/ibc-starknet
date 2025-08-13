use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::Index;
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::relayer_components::multi::traits::birelay_at::{
    BiRelayTypeProviderAtComponent, HasBiRelayTypeAt,
};
use hermes_core::relayer_components::multi::traits::chain_at::{
    ChainTypeProviderAtComponent, HasChainTypeAt,
};
use hermes_core::relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_core::relayer_components::multi::traits::relay_at::{
    HasRelayTypeAt, RelayTypeProviderAtComponent,
};
use hermes_core::test_components::driver::traits::{
    ChainDriverGetterAtComponent, ChainDriverTypeProviderAtComponent, ChannelIdGetterAtComponent,
    HasChainDriverTypeAt, RelayDriverGetterAtComponent, RelayDriverTypeProviderAtComponent,
};
use hermes_core::test_components::setup::traits::{
    BinaryChannelDriverBuilder, BinaryChannelDriverBuilderComponent,
    CreateClientMessageOptionsGetterAtComponent, CreateClientPayloadOptionsGetterAtComponent,
    HasTestDriverType, PortIdGetterAtComponent, RecoverClientPayloadOptionsGetterAtComponent,
};
use hermes_core::test_components::test_case::traits::recover_client::RecoverClientHandlerComponent;
use hermes_core::test_components::test_case::traits::upgrade_client::{
    SetupUpgradeClientTestHandlerComponent, UpgradeClientHandlerComponent,
};
use hermes_cosmos::chain_components::impls::CosmosRecoverClientPayload;
use hermes_cosmos::chain_components::types::CosmosCreateClientOptions;
use hermes_cosmos::error::handlers::DebugError;
use hermes_cosmos::error::impls::UseHermesError;
use hermes_cosmos::integration_tests::contexts::CosmosChainDriver;
use hermes_cosmos::relayer::contexts::CosmosChain;
use hermes_cosmos::tracing_logging_components::contexts::TracingLogger;
use hermes_prelude::*;
use hermes_starknet_chain_components::impls::StarknetRecoverClientPayload;
use hermes_starknet_chain_components::types::{
    ChannelId, ClientId, ConnectionId, StarknetCreateClientPayloadOptions,
};
use hermes_starknet_chain_context::contexts::StarknetChain;
use hermes_starknet_relayer::contexts::{
    CosmosStarknetBiRelay, CosmosToStarknetRelay, StarknetCosmosBiRelay, StarknetToCosmosRelay,
};
use hermes_starknet_test_components::impls::RecoverStarknetClientHandler;
use ibc::core::host::types::identifiers::PortId;

use super::{CosmosStarknetRelayDriver, StarknetCosmosRelayDriver};
use crate::contexts::{
    SetupStarknetUpgradeClientTest, StarknetChainDriver, StarknetHandleUpgradeClient,
};

#[cgp_context(StarknetTestDriverComponents)]
#[derive(HasField)]
pub struct StarknetTestDriver {
    pub relay_driver_a_b: StarknetCosmosRelayDriver,
    pub relay_driver_b_a: CosmosStarknetRelayDriver,
    pub starknet_chain_driver: StarknetChainDriver,
    pub cosmos_chain_driver: CosmosChainDriver,
    pub client_id_a: ClientId,
    pub client_id_b: ClientId,
    pub connection_id_a: ConnectionId,
    pub connection_id_b: ConnectionId,
    pub channel_id_a: ChannelId,
    pub channel_id_b: ChannelId,
    pub port_id_a: PortId,
    pub port_id_b: PortId,
    pub create_client_payload_options_a: StarknetCreateClientPayloadOptions,
    pub create_client_payload_options_b: CosmosCreateClientOptions,
    pub create_client_message_options_a: (),
    pub create_client_message_options_b: (),
    pub recover_client_payload_options_a: StarknetRecoverClientPayload,
    pub recover_client_payload_options_b: CosmosRecoverClientPayload,
}

delegate_components! {
    StarknetTestDriverComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        LoggerComponent:
            TracingLogger,
        RecoverClientHandlerComponent:
            RecoverStarknetClientHandler,
        SetupUpgradeClientTestHandlerComponent:
            SetupStarknetUpgradeClientTest,
        UpgradeClientHandlerComponent:
            StarknetHandleUpgradeClient,
        ChainTypeProviderAtComponent<Index<0>>:
            UseType<StarknetChain>,
        ChainTypeProviderAtComponent<Index<1>>:
            UseType<CosmosChain>,
        RelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<StarknetToCosmosRelay>,
        RelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosToStarknetRelay>,
        BiRelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<StarknetCosmosBiRelay>,
        BiRelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosStarknetBiRelay>,
        ChainDriverTypeProviderAtComponent<Index<0>>:
            UseType<StarknetChainDriver>,
        ChainDriverTypeProviderAtComponent<Index<1>>:
            UseType<CosmosChainDriver>,
        RelayDriverTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<StarknetCosmosRelayDriver>,
        RelayDriverTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosStarknetRelayDriver>,
        ChainDriverGetterAtComponent<Index<0>>:
            UseField<symbol!("starknet_chain_driver")>,
        ChainDriverGetterAtComponent<Index<1>>:
            UseField<symbol!("cosmos_chain_driver")>,
        ClientIdAtGetterComponent<Index<0>, Index<1>>:
            UseField<symbol!("client_id_a")>,
        ClientIdAtGetterComponent<Index<1>, Index<0>>:
            UseField<symbol!("client_id_b")>,
        RelayDriverGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("relay_driver_a_b")>,
        RelayDriverGetterAtComponent<Index<1>, Index<0>>:
            UseField<symbol!("relay_driver_b_a")>,
        ChannelIdGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("channel_id_a")>,
        ChannelIdGetterAtComponent<Index<1>, Index<0>>:
            UseField<symbol!("channel_id_b")>,
        PortIdGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("port_id_a")>,
        PortIdGetterAtComponent<Index<1>, Index<0>>:
            UseField<symbol!("port_id_b")>,
        CreateClientPayloadOptionsGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("create_client_payload_options_a")>,
        CreateClientPayloadOptionsGetterAtComponent<Index<1>, Index<0>>:
            UseField<symbol!("create_client_payload_options_b")>,
        CreateClientMessageOptionsGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("create_client_message_options_a")>,
        CreateClientMessageOptionsGetterAtComponent<Index<1>, Index<0>>:
            UseField<symbol!("create_client_message_options_b")>,
        RecoverClientPayloadOptionsGetterAtComponent<Index<0>>:
            UseField<symbol!("recover_client_payload_options_a")>,
        RecoverClientPayloadOptionsGetterAtComponent<Index<1>>:
            UseField<symbol!("recover_client_payload_options_b")>,
    }
}

#[cgp_new_provider(BinaryChannelDriverBuilderComponent)]
impl<Setup> BinaryChannelDriverBuilder<Setup> for BuildStarknetTestDriver
where
    Setup: HasBiRelayTypeAt<Index<0>, Index<1>, BiRelay = StarknetCosmosBiRelay>
        + HasRelayTypeAt<Index<0>, Index<1>, Relay = StarknetToCosmosRelay>
        + HasRelayTypeAt<Index<1>, Index<0>, Relay = CosmosToStarknetRelay>
        + HasChainTypeAt<Index<0>, Chain = StarknetChain>
        + HasChainTypeAt<Index<1>, Chain = CosmosChain>
        + HasChainDriverTypeAt<Index<0>, ChainDriver = StarknetChainDriver>
        + HasChainDriverTypeAt<Index<1>, ChainDriver = CosmosChainDriver>
        + HasTestDriverType<TestDriver = StarknetTestDriver>
        + CanRaiseAsyncError<String>,
{
    async fn build_driver_with_binary_channel(
        _setup: &Setup,
        birelay: StarknetCosmosBiRelay,
        starknet_chain_driver: StarknetChainDriver,
        cosmos_chain_driver: CosmosChainDriver,
        client_id_a: ClientId,
        client_id_b: ClientId,
        connection_id_a: ConnectionId,
        connection_id_b: ConnectionId,
        channel_id_a: ChannelId,
        channel_id_b: ChannelId,
        port_id_a: PortId,
        port_id_b: PortId,
        create_client_payload_options_a: &StarknetCreateClientPayloadOptions,
        create_client_payload_options_b: &CosmosCreateClientOptions,
    ) -> Result<StarknetTestDriver, Setup::Error> {
        let relay_driver_b_a = CosmosStarknetRelayDriver {
            birelay: CosmosStarknetBiRelay {
                runtime: birelay.runtime.clone(),
                relay_a_to_b: birelay.relay_b_to_a.clone(),
                relay_b_to_a: birelay.relay_a_to_b.clone(),
            },
        };

        // TODO: These are hardcoded values for Osmosis v28.0.0
        let cosmos_recover_client_payload = CosmosRecoverClientPayload {
            deposit_amount: 110000,
            deposit_denom: "stake".to_owned(),
        };

        // TODO: These are hardcoded values for Osmosis v28.0.0
        let starknet_recover_client_payload = StarknetRecoverClientPayload;

        let relay_driver_a_b = StarknetCosmosRelayDriver { birelay };

        let driver = StarknetTestDriver {
            relay_driver_a_b,
            relay_driver_b_a,
            starknet_chain_driver,
            cosmos_chain_driver,
            client_id_a,
            client_id_b,
            connection_id_a,
            connection_id_b,
            channel_id_a,
            channel_id_b,
            port_id_a,
            port_id_b,
            create_client_payload_options_a: create_client_payload_options_a.clone(),
            create_client_payload_options_b: create_client_payload_options_b.clone(),
            create_client_message_options_a: (),
            create_client_message_options_b: (),
            recover_client_payload_options_a: starknet_recover_client_payload,
            recover_client_payload_options_b: cosmos_recover_client_payload,
        };

        Ok(driver)
    }
}
