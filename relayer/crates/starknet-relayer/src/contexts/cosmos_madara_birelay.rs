use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::core::field::Index;
use cgp::extra::run::RunnerComponent;
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::relayer_components::birelay::traits::AutoBiRelayerComponent;
use hermes_core::relayer_components::components::default::DefaultBiRelayComponents;
use hermes_core::relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::relay_at::{
    RelayGetterAtComponent, RelayTypeProviderAtComponent,
};
use hermes_core::runtime_components::traits::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_cosmos::error::impls::UseHermesError;
use hermes_cosmos::relayer::contexts::CosmosChain;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_cosmos::tracing_logging_components::contexts::TracingLogger;
use hermes_prelude::*;
use hermes_starknet_madara_tests::contexts::MadaraChain;
use hermes_starknet_madara_tests::impls::HandleMadaraChainError;

use crate::contexts::{CosmosToMadaraRelay, MadaraToCosmosRelay};

#[cgp_context(CosmosMadaraBiRelayComponents: DefaultBiRelayComponents)]
#[derive(Clone, HasField)]
pub struct CosmosMadaraBiRelay {
    pub runtime: HermesRuntime,
    pub relay_a_to_b: CosmosToMadaraRelay,
    pub relay_b_to_a: MadaraToCosmosRelay,
}

delegate_components! {
    CosmosMadaraBiRelayComponents {
        [
            ErrorTypeProviderComponent,
            ErrorWrapperComponent,
        ]: UseHermesError,
        ErrorRaiserComponent:
            UseDelegate<HandleMadaraChainError>,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        LoggerComponent:
            TracingLogger,
        ChainTypeProviderAtComponent<Index<0>>:
            UseType<CosmosChain>,
        ChainTypeProviderAtComponent<Index<1>>:
            UseType<MadaraChain>,
        RelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<CosmosToMadaraRelay>,
        RelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<MadaraToCosmosRelay>,
        RelayGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("relay_a_to_b")>,
        RelayGetterAtComponent<Index<1>, Index<0>>:
            UseField<symbol!("relay_b_to_a")>,
    }
}

pub trait CanUseCosmosMadaraBiRelay:
    CanUseComponent<RunnerComponent> + CanUseComponent<AutoBiRelayerComponent>
{
}

impl CanUseCosmosMadaraBiRelay for CosmosMadaraBiRelay {}
