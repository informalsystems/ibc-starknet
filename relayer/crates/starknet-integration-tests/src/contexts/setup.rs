use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_cosmos_integration_tests::contexts::chain_driver::CosmosChainDriver;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_relayer::presets::relay::ChainTypeAtComponent;
use hermes_test_components::driver::traits::types::chain_driver_at::{
    ChainDriverTypeAtComponent, HasChainDriverTypeAt,
};
use hermes_test_components::setup::binary_channel::components::{
    BinaryChannelTestComponents, IsBinaryChannelTestComponents,
};
use hermes_test_components::setup::binary_channel::impls::fields::UseBinarySetupFields;
use hermes_test_components::setup::traits::bootstrap_at::{BootstrapAtComponent, HasBootstrapAt};

use crate::contexts::bootstrap::StarknetBootstrap;
use crate::contexts::chain_driver::StarknetChainDriver;
use crate::contexts::osmosis_bootstrap::OsmosisBootstrap;

#[derive(HasField)]
pub struct StarknetSetup {
    pub bootstrap_a: OsmosisBootstrap,
    pub bootstrap_b: StarknetBootstrap,
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
    }
}

pub trait CanUseStarknetSetup:
    HasBootstrapAt<Index<0>, Bootstrap = OsmosisBootstrap>
    + HasBootstrapAt<Index<1>, Bootstrap = StarknetBootstrap>
    + HasChainTypeAt<Index<0>, Chain = CosmosChain>
    + HasChainTypeAt<Index<1>, Chain = StarknetChain>
    + HasChainDriverTypeAt<Index<0>, ChainDriver = CosmosChainDriver>
    + HasChainDriverTypeAt<Index<1>, ChainDriver = StarknetChainDriver>
{
}

impl CanUseStarknetSetup for StarknetSetup {}
