use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::Index;
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::relayer_components::multi::traits::birelay_at::{
    BiRelayTypeProviderAtComponent, HasBiRelayTypeAt,
};
use hermes_core::relayer_components::multi::traits::chain_at::{
    ChainTypeProviderAtComponent, HasChainTypeAt,
};
use hermes_core::relayer_components::multi::traits::relay_at::{
    HasRelayTypeAt, RelayTypeProviderAtComponent,
};
use hermes_core::test_components::driver::traits::{
    ChainDriverGetterAtComponent, ChainDriverTypeProviderAtComponent, ChannelIdGetterAtComponent,
    HasChainDriverTypeAt, RelayDriverGetterAtComponent, RelayDriverTypeProviderAtComponent,
};
use hermes_core::test_components::setup::traits::{
    BinaryChannelDriverBuilder, BinaryChannelDriverBuilderComponent, HasTestDriverType,
    PortIdGetterAtComponent,
};
use hermes_cosmos::error::handlers::DebugError;
use hermes_cosmos::error::impls::UseHermesError;
use hermes_cosmos::integration_tests::contexts::CosmosChainDriver;
use hermes_cosmos::relayer::contexts::CosmosChain;
use hermes_cosmos::tracing_logging_components::contexts::TracingLogger;
use hermes_prelude::*;
use hermes_starknet_chain_components::types::{ChannelId, ConnectionId};
use hermes_starknet_chain_context::contexts::StarknetChain;
use hermes_starknet_relayer::contexts::cosmos_starknet_birelay::CosmosStarknetBiRelay;
use hermes_starknet_relayer::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use hermes_starknet_relayer::contexts::starknet_cosmos_birelay::StarknetCosmosBiRelay;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use ibc::core::host::types::identifiers::PortId;

use crate::contexts::chain_driver::StarknetChainDriver;
use crate::contexts::cosmos_starknet_relay_driver::CosmosStarknetRelayDriver;
use crate::contexts::starknet_cosmos_relay_driver::StarknetCosmosRelayDriver;

#[cgp_context(StarknetTestDriverComponents)]
#[derive(HasField)]
pub struct StarknetTestDriver {
    pub relay_driver_a_b: StarknetCosmosRelayDriver,
    pub relay_driver_b_a: CosmosStarknetRelayDriver,
    pub starknet_chain_driver: StarknetChainDriver,
    pub cosmos_chain_driver: CosmosChainDriver,
    pub connection_id_a: ConnectionId,
    pub connection_id_b: ConnectionId,
    pub channel_id_a: ChannelId,
    pub channel_id_b: ChannelId,
    pub port_id_a: PortId,
    pub port_id_b: PortId,
}

delegate_components! {
    StarknetTestDriverComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        LoggerComponent:
            TracingLogger,
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
        + HasAsyncErrorType,
{
    async fn build_driver_with_binary_channel(
        _setup: &Setup,
        birelay: StarknetCosmosBiRelay,
        starknet_chain_driver: StarknetChainDriver,
        cosmos_chain_driver: CosmosChainDriver,
        connection_id_a: ConnectionId,
        connection_id_b: ConnectionId,
        channel_id_a: ChannelId,
        channel_id_b: ChannelId,
        port_id_a: PortId,
        port_id_b: PortId,
    ) -> Result<StarknetTestDriver, Setup::Error> {
        let relay_driver_b_a = CosmosStarknetRelayDriver {
            birelay: CosmosStarknetBiRelay {
                runtime: birelay.runtime.clone(),
                relay_a_to_b: birelay.relay_b_to_a.clone(),
                relay_b_to_a: birelay.relay_a_to_b.clone(),
            },
        };

        let relay_driver_a_b = StarknetCosmosRelayDriver { birelay };

        let driver = StarknetTestDriver {
            relay_driver_a_b,
            relay_driver_b_a,
            starknet_chain_driver,
            cosmos_chain_driver,
            connection_id_a,
            connection_id_b,
            channel_id_a,
            channel_id_b,
            port_id_a,
            port_id_b,
        };

        Ok(driver)
    }
}
