use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::core::field::Index;
use cgp::extra::run::RunnerComponent;
use cgp::prelude::*;
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
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;

use crate::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use crate::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;

#[cgp_context(StarknetCosmosBiRelayComponents: DefaultBiRelayComponents)]
#[derive(Clone, HasField)]
pub struct CosmosStarknetBiRelay {
    pub runtime: HermesRuntime,
    pub relay_a_to_b: CosmosToStarknetRelay,
    pub relay_b_to_a: StarknetToCosmosRelay,
}

delegate_components! {
    StarknetCosmosBiRelayComponents {
        [
            ErrorTypeProviderComponent,
            ErrorWrapperComponent,
        ]: UseHermesError,
        ErrorRaiserComponent:
            UseDelegate<HandleStarknetChainError>,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        LoggerComponent:
            TracingLogger,
        ChainTypeProviderAtComponent<Index<0>>:
            UseType<CosmosChain>,
        ChainTypeProviderAtComponent<Index<1>>:
            UseType<StarknetChain>,
        RelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<CosmosToStarknetRelay>,
        RelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<StarknetToCosmosRelay>,
        RelayGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("relay_a_to_b")>,
        RelayGetterAtComponent<Index<1>, Index<0>>:
            UseField<symbol!("relay_b_to_a")>,
    }
}

pub trait CanUseCosmosStarnetBiRelay:
    CanUseComponent<RunnerComponent> + CanUseComponent<AutoBiRelayerComponent>
{
}

impl CanUseCosmosStarnetBiRelay for CosmosStarknetBiRelay {}
