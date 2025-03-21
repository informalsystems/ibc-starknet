use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_cosmos_integration_tests::contexts::chain_driver::CosmosChainDriver;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::HasRelayTypeAt;
use hermes_starknet_chain_components::types::channel_id::ChannelId;
use hermes_starknet_chain_components::types::connection_id::ConnectionId;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_relayer::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use hermes_starknet_relayer::contexts::starknet_cosmos_birelay::StarknetCosmosBiRelay;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::driver::traits::types::chain_driver_at::HasChainDriverTypeAt;
use hermes_test_components::setup::traits::driver::HasTestDriverType;
use hermes_test_components::setup::traits::drivers::binary_channel::{
    BinaryChannelDriverBuilder, BinaryChannelDriverBuilderComponent,
};
use ibc::core::host::types::identifiers::PortId;

use crate::contexts::chain_driver::StarknetChainDriver;
use crate::contexts::relay_driver::StarknetRelayDriver;

#[derive(HasField)]
pub struct StarknetTestDriver {
    pub relay_driver: StarknetRelayDriver,
    pub starknet_chain_driver: StarknetChainDriver,
    pub cosmos_chain_driver: CosmosChainDriver,
    pub connection_id_a: ConnectionId,
    pub connection_id_b: ConnectionId,
    pub channel_id_a: ChannelId,
    pub channel_id_b: ChannelId,
    pub port_id_a: PortId,
    pub port_id_b: PortId,
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
        let relay_driver = StarknetRelayDriver { birelay };

        let driver = StarknetTestDriver {
            relay_driver,
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
