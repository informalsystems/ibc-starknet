use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::Index;
use hermes_core::relayer_components::multi::traits::birelay_at::BiRelayTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::relay_at::RelayTypeProviderAtComponent;
use hermes_core::test_components::driver::traits::{
    BuilderAtTypeProviderComponent, ChainDriverTypeProviderAtComponent,
};
use hermes_core::test_components::setup::binary_channel::BinaryChannelTestComponents;
use hermes_core::test_components::setup::traits::{
    BiRelaySetupComponent, BinaryChannelDriverBuilderComponent, BootstrapGetterAtComponent,
    BootstrapTypeProviderAtComponent, BuilderAtGetterComponent, ChainSetupComponent,
    ChannelSetupComponent, ClientSetupComponent, ConnectionSetupComponent,
    CreateClientMessageOptionsGetterAtComponent, CreateClientPayloadOptionsGetterAtComponent,
    DriverBuilderComponent, InitChannelOptionsGetterAtComponent,
    InitConnectionOptionsGetterAtComponent, PortIdGetterAtComponent, RelaySetupComponent,
    TestDriverTypeProviderComponent,
};
use hermes_cosmos::chain_components::types::{
    CosmosCreateClientOptions, CosmosInitChannelOptions, CosmosInitConnectionOptions,
};
use hermes_cosmos::error::handlers::DebugError;
use hermes_cosmos::error::impls::UseHermesError;
use hermes_cosmos::integration_tests::contexts::CosmosChainDriver;
use hermes_cosmos::integration_tests::impls::UseCosmosInitChannelOptions;
use hermes_cosmos::relayer::contexts::CosmosChain;
use hermes_prelude::*;
use hermes_starknet_chain_components::types::StarknetCreateClientPayloadOptions;
use hermes_starknet_madara_tests::contexts::{MadaraBootstrap, MadaraChain, MadaraChainDriver};
use hermes_starknet_relayer::contexts::{
    CosmosMadaraBiRelay, CosmosToMadaraRelay, MadaraBuilder, MadaraCosmosBiRelay,
    MadaraToCosmosRelay,
};
use ibc::core::host::types::identifiers::PortId;

use super::{BuildMadaraTestDriver, MadaraTestDriver, OsmosisBootstrap};

#[cgp_context(MadaraBinaryChannelSetupComponents: BinaryChannelTestComponents)]
#[derive(HasField)]
pub struct MadaraTestSetup {
    pub madara_bootstrap: MadaraBootstrap,
    pub osmosis_bootstrap: OsmosisBootstrap,
    pub madara_builder: MadaraBuilder,
    pub port_id: PortId,
    pub init_channel_options: CosmosInitChannelOptions,
    pub init_connection_options: CosmosInitConnectionOptions,
    pub cosmos_create_client_payload_options: CosmosCreateClientOptions,
    pub starknet_create_client_payload_options: StarknetCreateClientPayloadOptions,
    pub create_client_message_options: (),
}

impl MadaraTestSetup {
    pub fn new_with_defaults(
        madara_bootstrap: MadaraBootstrap,
        osmosis_bootstrap: OsmosisBootstrap,
        madara_builder: MadaraBuilder,
        wasm_code_hash: [u8; 32],
    ) -> Self {
        Self {
            madara_bootstrap,
            osmosis_bootstrap,
            madara_builder,
            starknet_create_client_payload_options: StarknetCreateClientPayloadOptions {
                wasm_code_hash,
            },
            port_id: PortId::transfer(),
            init_channel_options: Default::default(),
            init_connection_options: Default::default(),
            cosmos_create_client_payload_options: Default::default(),
            create_client_message_options: (),
        }
    }
}

delegate_components! {
    MadaraBinaryChannelSetupComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        TestDriverTypeProviderComponent:
            UseType<MadaraTestDriver>,
        BootstrapTypeProviderAtComponent<Index<0>>:
            UseType<MadaraBootstrap>,
        BootstrapGetterAtComponent<Index<0>>:
            UseField<symbol!("madara_bootstrap")>,
        BootstrapTypeProviderAtComponent<Index<1>>:
            UseType<OsmosisBootstrap>,
        BootstrapGetterAtComponent<Index<1>>:
            UseField<symbol!("osmosis_bootstrap")>,
        ChainTypeProviderAtComponent<Index<0>>:
            UseType<MadaraChain>,
        ChainTypeProviderAtComponent<Index<1>>:
            UseType<CosmosChain>,
        ChainDriverTypeProviderAtComponent<Index<0>>:
            UseType<MadaraChainDriver>,
        ChainDriverTypeProviderAtComponent<Index<1>>:
            UseType<CosmosChainDriver>,
        [
            BuilderAtTypeProviderComponent<Index<0>, Index<1>>,
            BuilderAtTypeProviderComponent<Index<1>, Index<0>>,
        ]:
            UseType<MadaraBuilder>,
        [
            BuilderAtGetterComponent<Index<0>, Index<1>>,
            BuilderAtGetterComponent<Index<1>, Index<0>>,
        ]:
            UseField<symbol!("madara_builder")>,
        RelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<MadaraToCosmosRelay>,
        RelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosToMadaraRelay>,
        BiRelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<MadaraCosmosBiRelay>,
        BiRelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosMadaraBiRelay>,
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
        CreateClientPayloadOptionsGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("starknet_create_client_payload_options")>,
        CreateClientPayloadOptionsGetterAtComponent<Index<1>, Index<0>>:
            UseField<symbol!("cosmos_create_client_payload_options")>,
        [
            InitChannelOptionsGetterAtComponent<Index<0>, Index<1>>,
            InitChannelOptionsGetterAtComponent<Index<1>, Index<0>>,
        ]:
            UseCosmosInitChannelOptions<symbol!("init_channel_options")>,
        BinaryChannelDriverBuilderComponent:
            BuildMadaraTestDriver,
    }
}

check_components! {
    CanUseStarketTestSetup for MadaraTestSetup {
        ChainSetupComponent: [
            Index<0>,
            Index<1>,
        ],
        RelaySetupComponent: [
            (Index<0>, Index<1>),
            (Index<1>, Index<0>),
        ],
        BiRelaySetupComponent: [
            (Index<0>, Index<1>),
        ],
        ClientSetupComponent: [
            (Index<0>, Index<1>),
            (Index<1>, Index<0>),
        ],
        ConnectionSetupComponent: [
            (Index<0>, Index<1>),
            (Index<1>, Index<0>),
        ],
        ChannelSetupComponent: [
            (Index<0>, Index<1>),
            (Index<1>, Index<0>),
        ],
        DriverBuilderComponent,
    }
}
