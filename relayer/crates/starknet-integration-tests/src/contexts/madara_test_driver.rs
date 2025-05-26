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
use hermes_starknet_madara_tests::contexts::{MadaraChain, MadaraChainDriver};
use hermes_starknet_relayer::contexts::{
    CosmosMadaraBiRelay, CosmosToMadaraRelay, MadaraCosmosBiRelay, MadaraToCosmosRelay,
};
use ibc::core::host::types::identifiers::PortId;

use super::{CosmosMadaraRelayDriver, MadaraCosmosRelayDriver};

#[cgp_context(MadaraTestDriverComponents)]
#[derive(HasField)]
pub struct MadaraTestDriver {
    pub relay_driver_a_b: MadaraCosmosRelayDriver,
    pub relay_driver_b_a: CosmosMadaraRelayDriver,
    pub madara_chain_driver: MadaraChainDriver,
    pub cosmos_chain_driver: CosmosChainDriver,
    pub connection_id_a: ConnectionId,
    pub connection_id_b: ConnectionId,
    pub channel_id_a: ChannelId,
    pub channel_id_b: ChannelId,
    pub port_id_a: PortId,
    pub port_id_b: PortId,
}

delegate_components! {
    MadaraTestDriverComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        LoggerComponent:
            TracingLogger,
        ChainTypeProviderAtComponent<Index<0>>:
            UseType<MadaraChain>,
        ChainTypeProviderAtComponent<Index<1>>:
            UseType<CosmosChain>,
        RelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<MadaraToCosmosRelay>,
        RelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosToMadaraRelay>,
        BiRelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<MadaraCosmosBiRelay>,
        BiRelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosMadaraBiRelay>,
        ChainDriverTypeProviderAtComponent<Index<0>>:
            UseType<MadaraChainDriver>,
        ChainDriverTypeProviderAtComponent<Index<1>>:
            UseType<CosmosChainDriver>,
        RelayDriverTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<MadaraCosmosRelayDriver>,
        RelayDriverTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosMadaraRelayDriver>,
        ChainDriverGetterAtComponent<Index<0>>:
            UseField<symbol!("madara_chain_driver")>,
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
impl<Setup> BinaryChannelDriverBuilder<Setup> for BuildMadaraTestDriver
where
    Setup: HasBiRelayTypeAt<Index<0>, Index<1>, BiRelay = MadaraCosmosBiRelay>
        + HasRelayTypeAt<Index<0>, Index<1>, Relay = MadaraToCosmosRelay>
        + HasRelayTypeAt<Index<1>, Index<0>, Relay = CosmosToMadaraRelay>
        + HasChainTypeAt<Index<0>, Chain = MadaraChain>
        + HasChainTypeAt<Index<1>, Chain = CosmosChain>
        + HasChainDriverTypeAt<Index<0>, ChainDriver = MadaraChainDriver>
        + HasChainDriverTypeAt<Index<1>, ChainDriver = CosmosChainDriver>
        + HasTestDriverType<TestDriver = MadaraTestDriver>
        + HasAsyncErrorType,
{
    async fn build_driver_with_binary_channel(
        _setup: &Setup,
        birelay: MadaraCosmosBiRelay,
        madara_chain_driver: MadaraChainDriver,
        cosmos_chain_driver: CosmosChainDriver,
        connection_id_a: ConnectionId,
        connection_id_b: ConnectionId,
        channel_id_a: ChannelId,
        channel_id_b: ChannelId,
        port_id_a: PortId,
        port_id_b: PortId,
    ) -> Result<MadaraTestDriver, Setup::Error> {
        tracing::warn!("called build_driver_with_binary_channel");
        let relay_driver_b_a = CosmosMadaraRelayDriver {
            birelay: CosmosMadaraBiRelay {
                runtime: birelay.runtime.clone(),
                relay_a_to_b: birelay.relay_b_to_a.clone(),
                relay_b_to_a: birelay.relay_a_to_b.clone(),
            },
        };

        let relay_driver_a_b = MadaraCosmosRelayDriver { birelay };

        let driver = MadaraTestDriver {
            relay_driver_a_b,
            relay_driver_b_a,
            madara_chain_driver,
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
