use core::marker::PhantomData;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::Index;
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientOptions;
use hermes_cosmos_integration_tests::contexts::chain_driver::CosmosChainDriver;
use hermes_cosmos_integration_tests::impls::init_channel_options::UseCosmosInitChannelOptions;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::{HasRelayTypeAt, RelayTypeAtComponent};
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_relayer::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_starknet_relayer::presets::relay::ChainTypeAtComponent;
use hermes_test_components::driver::traits::types::chain_driver_at::{
    ChainDriverTypeAtComponent, HasChainDriverTypeAt,
};
use hermes_test_components::setup::binary_channel::components::{
    BinaryChannelTestComponents, IsBinaryChannelTestComponents,
};
use hermes_test_components::setup::binary_channel::impls::fields::UseBinarySetupFields;
use hermes_test_components::setup::traits::bootstrap_at::{BootstrapAtComponent, HasBootstrapAt};
use hermes_test_components::setup::traits::chain::CanSetupChain;
use hermes_test_components::setup::traits::clients::CanSetupClients;
use hermes_test_components::setup::traits::create_client_options_at::{
    ProvideCreateClientMessageOptionsAt, ProvideCreateClientPayloadOptionsAt,
};
use hermes_test_components::setup::traits::init_channel_options_at::InitChannelOptionsAtComponent;
use hermes_test_components::setup::traits::init_connection_options_at::InitConnectionOptionsAtComponent;

use crate::contexts::bootstrap::StarknetBootstrap;
use crate::contexts::chain_driver::StarknetChainDriver;
use crate::contexts::osmosis_bootstrap::OsmosisBootstrap;

#[derive(HasField)]
pub struct StarknetSetup {
    pub bootstrap_a: OsmosisBootstrap,
    pub bootstrap_b: StarknetBootstrap,
    pub init_channel_options: CosmosInitChannelOptions,
    pub init_connection_options: CosmosInitConnectionOptions,
    pub cosmos_create_client_payload_options: CosmosCreateClientOptions,
    pub starknet_create_client_payload_options: StarknetCreateClientPayloadOptions,
}

pub struct StarknetSetupComponents;

impl HasComponents for StarknetSetup {
    type Components = StarknetSetupComponents;
}

impl<Component> DelegateComponent<Component> for StarknetSetupComponents
where
    Self: IsBinaryChannelTestComponents<Component>,
{
    type Delegate = BinaryChannelTestComponents;
}

delegate_components! {
    StarknetSetupComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
        [
            BootstrapAtComponent,
            ChainTypeAtComponent<Index<0>>,
            ChainTypeAtComponent<Index<1>>,
            ChainDriverTypeAtComponent,
        ]: UseBinarySetupFields,
        RelayTypeAtComponent<Index<0>, Index<1>>: WithType<CosmosToStarknetRelay>,
        RelayTypeAtComponent<Index<1>, Index<0>>: WithType<StarknetToCosmosRelay>,
        InitConnectionOptionsAtComponent: UseField<symbol!("init_connection_options")>,
        InitChannelOptionsAtComponent: UseCosmosInitChannelOptions,
    }
}

impl ProvideCreateClientMessageOptionsAt<StarknetSetup, Index<0>, Index<1>>
    for StarknetSetupComponents
{
    fn create_client_message_options(
        _setup: &StarknetSetup,
        _index: PhantomData<(Index<0>, Index<1>)>,
    ) -> &() {
        &()
    }
}

impl ProvideCreateClientMessageOptionsAt<StarknetSetup, Index<1>, Index<0>>
    for StarknetSetupComponents
{
    fn create_client_message_options(
        _setup: &StarknetSetup,
        _index: PhantomData<(Index<1>, Index<0>)>,
    ) -> &() {
        &()
    }
}

impl ProvideCreateClientPayloadOptionsAt<StarknetSetup, Index<0>, Index<1>>
    for StarknetSetupComponents
{
    fn create_client_payload_options(
        setup: &StarknetSetup,
        _index: PhantomData<(Index<0>, Index<1>)>,
    ) -> &CosmosCreateClientOptions {
        &setup.cosmos_create_client_payload_options
    }
}

impl ProvideCreateClientPayloadOptionsAt<StarknetSetup, Index<1>, Index<0>>
    for StarknetSetupComponents
{
    fn create_client_payload_options(
        setup: &StarknetSetup,
        _index: PhantomData<(Index<1>, Index<0>)>,
    ) -> &StarknetCreateClientPayloadOptions {
        &setup.starknet_create_client_payload_options
    }
}

pub trait CanUseStarknetSetup:
    HasBootstrapAt<Index<0>, Bootstrap = OsmosisBootstrap>
    + HasBootstrapAt<Index<1>, Bootstrap = StarknetBootstrap>
    + HasChainTypeAt<Index<0>, Chain = CosmosChain>
    + HasChainTypeAt<Index<1>, Chain = StarknetChain>
    + HasChainDriverTypeAt<Index<0>, ChainDriver = CosmosChainDriver>
    + HasChainDriverTypeAt<Index<1>, ChainDriver = StarknetChainDriver>
    + HasRelayTypeAt<Index<0>, Index<1>, Relay = CosmosToStarknetRelay>
    + HasRelayTypeAt<Index<1>, Index<0>, Relay = StarknetToCosmosRelay>
    + CanSetupChain<Index<0>>
    + CanSetupChain<Index<1>>
    + CanSetupClients<Index<0>, Index<1>>
    + CanSetupClients<Index<1>, Index<0>>
{
}

impl CanUseStarknetSetup for StarknetSetup {}
