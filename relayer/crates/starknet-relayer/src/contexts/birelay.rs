use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::{Index, WithField};
use cgp::core::types::WithType;
use cgp::extra::run::CanRun;
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::impls::ProvideHermesError;
use hermes_relayer_components::birelay::traits::two_way::TwoWayRelayGetter;
use hermes_relayer_components::components::default::birelay::{
    DefaultBiRelayComponents, IsDefaultBiRelayComponents,
};
use hermes_relayer_components::multi::traits::chain_at::ChainTypeAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayTypeAtComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;

use crate::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use crate::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;

#[derive(Clone, HasField)]
pub struct StarknetCosmosBiRelay {
    pub runtime: HermesRuntime,
    pub relay_a_to_b: StarknetToCosmosRelay,
    pub relay_b_to_a: CosmosToStarknetRelay,
}

pub struct StarknetCosmosBiRelayComponents;

impl HasComponents for StarknetCosmosBiRelay {
    type Components = StarknetCosmosBiRelayComponents;
}

impl<Component> DelegateComponent<Component> for StarknetCosmosBiRelayComponents
where
    Self: IsDefaultBiRelayComponents<Component>,
{
    type Delegate = DefaultBiRelayComponents;
}

delegate_components! {
    StarknetCosmosBiRelayComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        RuntimeTypeComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        ChainTypeAtComponent<Index<0>>: WithType<StarknetChain>,
        ChainTypeAtComponent<Index<1>>: WithType<CosmosChain>,
        RelayTypeAtComponent<Index<0>, Index<1>>: WithType<StarknetToCosmosRelay>,
        RelayTypeAtComponent<Index<1>, Index<0>>: WithType<CosmosToStarknetRelay>,
    }
}

impl TwoWayRelayGetter<StarknetCosmosBiRelay> for StarknetCosmosBiRelayComponents {
    fn relay_a_to_b(birelay: &StarknetCosmosBiRelay) -> &StarknetToCosmosRelay {
        &birelay.relay_a_to_b
    }

    fn relay_b_to_a(birelay: &StarknetCosmosBiRelay) -> &CosmosToStarknetRelay {
        &birelay.relay_b_to_a
    }
}

pub trait CanUseCosmosStarnetBiRelay: CanRun {}

impl CanUseCosmosStarnetBiRelay for StarknetCosmosBiRelay {}
