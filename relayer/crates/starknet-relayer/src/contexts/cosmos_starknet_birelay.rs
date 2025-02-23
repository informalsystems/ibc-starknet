use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::{Index, WithField};
use cgp::core::types::WithType;
use cgp::extra::run::RunnerComponent;
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::impls::ProvideHermesError;
use hermes_relayer_components::birelay::traits::{
    AutoBiRelayerComponent, TwoWayRelayGetter, TwoWayRelayGetterComponent,
};
use hermes_relayer_components::components::default::birelay::DefaultBiRelayComponents;
use hermes_relayer_components::multi::traits::chain_at::ChainTypeAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayTypeAtComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
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
        ErrorTypeProviderComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        RuntimeTypeProviderComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        ChainTypeAtComponent<Index<0>>: WithType<CosmosChain>,
        ChainTypeAtComponent<Index<1>>: WithType<StarknetChain>,
        RelayTypeAtComponent<Index<0>, Index<1>>: WithType<CosmosToStarknetRelay>,
        RelayTypeAtComponent<Index<1>, Index<0>>: WithType<StarknetToCosmosRelay>,
    }
}

#[cgp_provider(TwoWayRelayGetterComponent)]
impl TwoWayRelayGetter<CosmosStarknetBiRelay> for StarknetCosmosBiRelayComponents {
    fn relay_a_to_b(birelay: &CosmosStarknetBiRelay) -> &CosmosToStarknetRelay {
        &birelay.relay_a_to_b
    }

    fn relay_b_to_a(birelay: &CosmosStarknetBiRelay) -> &StarknetToCosmosRelay {
        &birelay.relay_b_to_a
    }
}

pub trait CanUseCosmosStarnetBiRelay:
    CanUseComponent<RunnerComponent> + CanUseComponent<AutoBiRelayerComponent>
{
}

impl CanUseCosmosStarnetBiRelay for CosmosStarknetBiRelay {}
