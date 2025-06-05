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

#[cgp_context(MadaraCosmosBiRelayComponents: DefaultBiRelayComponents)]
#[derive(Clone, HasField)]
pub struct MadaraCosmosBiRelay {
    pub runtime: HermesRuntime,
    pub relay_a_to_b: MadaraToCosmosRelay,
    pub relay_b_to_a: CosmosToMadaraRelay,
}

delegate_components! {
    MadaraCosmosBiRelayComponents {
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
            UseType<MadaraChain>,
        ChainTypeProviderAtComponent<Index<1>>:
            UseType<CosmosChain>,
        RelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<MadaraToCosmosRelay>,
        RelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosToMadaraRelay>,
        RelayGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("relay_a_to_b")>,
        RelayGetterAtComponent<Index<1>, Index<0>>:
            UseField<symbol!("relay_b_to_a")>,
    }
}

pub trait CanUseMadaraCosmosBiRelay:
    CanUseComponent<RunnerComponent> + CanUseComponent<AutoBiRelayerComponent>
{
}

impl CanUseMadaraCosmosBiRelay for MadaraCosmosBiRelay {}
