use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientOptions;
use hermes_cosmos_integration_tests::contexts::chain_driver::CosmosChainDriver;
use hermes_cosmos_integration_tests::impls::init_channel_options::UseCosmosInitChannelOptions;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_relayer_components::multi::traits::birelay_at::BiRelayTypeProviderAtComponent;
use hermes_relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayTypeProviderAtComponent;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_relayer::contexts::builder::StarknetBuilder;
use hermes_starknet_relayer::contexts::cosmos_starknet_birelay::CosmosStarknetBiRelay;
use hermes_starknet_relayer::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use hermes_starknet_relayer::contexts::starknet_cosmos_birelay::StarknetCosmosBiRelay;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::driver::traits::types::builder_at::BuilderAtTypeProviderComponent;
use hermes_test_components::driver::traits::types::chain_driver_at::ChainDriverTypeProviderAtComponent;
use hermes_test_components::setup::binary_channel::components::BinaryChannelTestComponents;
use hermes_test_components::setup::traits::birelay::BiRelaySetupComponent;
use hermes_test_components::setup::traits::bootstrap_at::{
    BootstrapGetterAtComponent, BootstrapTypeProviderAtComponent,
};
use hermes_test_components::setup::traits::builder_at::BuilderAtGetterComponent;
use hermes_test_components::setup::traits::chain::ChainSetupComponent;
use hermes_test_components::setup::traits::channel::ChannelSetupComponent;
use hermes_test_components::setup::traits::clients::ClientSetupComponent;
use hermes_test_components::setup::traits::connection::ConnectionSetupComponent;
use hermes_test_components::setup::traits::create_client_options_at::{
    CreateClientMessageOptionsGetterAtComponent, CreateClientPayloadOptionsGetterAtComponent,
};
use hermes_test_components::setup::traits::driver::{
    DriverBuilderComponent, TestDriverTypeProviderComponent,
};
use hermes_test_components::setup::traits::drivers::binary_channel::BinaryChannelDriverBuilderComponent;
use hermes_test_components::setup::traits::init_channel_options_at::InitChannelOptionsGetterAtComponent;
use hermes_test_components::setup::traits::init_connection_options_at::InitConnectionOptionsGetterAtComponent;
use hermes_test_components::setup::traits::port_id_at::PortIdGetterAtComponent;
use hermes_test_components::setup::traits::relay::RelaySetupComponent;
use ibc::core::host::types::identifiers::PortId;

use crate::contexts::chain_driver::StarknetChainDriver;
use crate::contexts::osmosis_bootstrap::OsmosisBootstrap;
use crate::contexts::starknet_bootstrap::StarknetBootstrap;
use crate::contexts::test_driver::{BuildStarknetTestDriver, StarknetTestDriver};

#[cgp_context(StarknetBinaryChannelSetupComponents: BinaryChannelTestComponents)]
#[derive(HasField)]
pub struct StarknetTestSetup {
    pub starknet_bootstrap: StarknetBootstrap,
    pub osmosis_bootstrap: OsmosisBootstrap,
    pub cosmos_builder: CosmosBuilder,
    pub starknet_builder: StarknetBuilder,
    pub port_id: PortId,
    pub init_channel_options: CosmosInitChannelOptions,
    pub init_connection_options: CosmosInitConnectionOptions,
    pub cosmos_create_client_payload_options: CosmosCreateClientOptions,
    pub starknet_create_client_payload_options: StarknetCreateClientPayloadOptions,
    pub create_client_message_options: (),
}

delegate_components! {
    StarknetBinaryChannelSetupComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        TestDriverTypeProviderComponent:
            UseType<StarknetTestDriver>,
        BootstrapTypeProviderAtComponent<Index<0>>:
            UseType<StarknetBootstrap>,
        BootstrapGetterAtComponent<Index<0>>:
            UseField<symbol!("starknet_bootstrap")>,
        BootstrapTypeProviderAtComponent<Index<1>>:
            UseType<OsmosisBootstrap>,
        BootstrapGetterAtComponent<Index<1>>:
            UseField<symbol!("osmosis_bootstrap")>,
        ChainTypeProviderAtComponent<Index<0>>:
            UseType<StarknetChain>,
        ChainTypeProviderAtComponent<Index<1>>:
            UseType<CosmosChain>,
        ChainDriverTypeProviderAtComponent<Index<0>>:
            UseType<StarknetChainDriver>,
        ChainDriverTypeProviderAtComponent<Index<1>>:
            UseType<CosmosChainDriver>,
        BuilderAtTypeProviderComponent<Index<0>, Index<1>>:
            UseType<StarknetBuilder>,
        BuilderAtTypeProviderComponent<Index<1>, Index<0>>:
            UseType<CosmosBuilder>,
        BuilderAtGetterComponent<Index<0>, Index<1>>:
            UseField<symbol!("starknet_builder")>,
        BuilderAtGetterComponent<Index<1>, Index<0>>:
            UseField<symbol!("cosmos_builder")>,
        RelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<StarknetToCosmosRelay>,
        RelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosToStarknetRelay>,
        BiRelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<StarknetCosmosBiRelay>,
        BiRelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosStarknetBiRelay>,
        [
            PortIdGetterAtComponent<Index<0>, Index<1>>,
            PortIdGetterAtComponent<Index<1>, Index<0>>,
        ]:
            UseField<symbol!("port_id")>,
        [
            InitConnectionOptionsGetterAtComponent<Index<0>, Index<1>>,
            InitConnectionOptionsGetterAtComponent<Index<1>, Index<0>>,
        ]: UseField<symbol!("init_connection_options")>,
        [
            CreateClientMessageOptionsGetterAtComponent<Index<0>, Index<1>>,
            CreateClientMessageOptionsGetterAtComponent<Index<1>, Index<0>>,
        ]: UseField<symbol!("create_client_message_options")>,
        [
            CreateClientPayloadOptionsGetterAtComponent<Index<0>, Index<1>>,
            CreateClientPayloadOptionsGetterAtComponent<Index<1>, Index<0>>,
        ]: UseField<symbol!("create_client_payload_options")>,
        [
            InitChannelOptionsGetterAtComponent<Index<0>, Index<1>>,
            InitChannelOptionsGetterAtComponent<Index<1>, Index<0>>,
        ]:
            UseCosmosInitChannelOptions<symbol!("init_channel_options")>,
        BinaryChannelDriverBuilderComponent:
            BuildStarknetTestDriver,
    }
}

check_components! {
    CanUseStarketTestSetup for StarknetTestSetup {
        ChainSetupComponent: [
            Index<0>,
            Index<1>,
        ],
        // RelaySetupComponent: [
        //     (Index<0>, Index<1>),
        //     (Index<1>, Index<0>),
        // ],
        // BiRelaySetupComponent: [
        //     (Index<0>, Index<1>),
        // ],
        // ClientSetupComponent: [
        //     (Index<0>, Index<1>),
        //     (Index<1>, Index<0>),
        // ],
        ConnectionSetupComponent: [
            (Index<0>, Index<1>),
            (Index<1>, Index<0>),
        ],
        ChannelSetupComponent: [
            (Index<0>, Index<1>),
            (Index<1>, Index<0>),
        ],
        // DriverBuilderComponent,
    }
}
