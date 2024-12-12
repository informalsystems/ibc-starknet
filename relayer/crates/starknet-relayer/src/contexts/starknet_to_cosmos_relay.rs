use core::ops::Deref;
use std::sync::Arc;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::impls::use_field::{UseField, WithField};
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::impls::ProvideHermesError;
use hermes_logger::ProvideHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::components::default::relay::{
    DefaultRelayPreset, IsDefaultRelayPreset,
};
use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeAtComponent,
};
use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_relayer_components::multi::types::index::Index;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::impls::selector::SelectRelayBToA;
use hermes_relayer_components::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{
    DestinationTarget, HasDestinationTargetChainTypes, HasSourceTargetChainTypes,
    HasTargetClientIds, SourceTarget,
};
use hermes_relayer_components::relay::traits::update_client_message_builder::{
    CanBuildTargetUpdateClientMessage, CanSendTargetUpdateClientMessage,
};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};
use hermes_starknet_chain_context::contexts::chain::StarknetChain;

use crate::contexts::cosmos_to_starknet_relay::{
    CosmosToStarknetRelayFields, HasCosmosToStarknetRelayFields,
};
use crate::impls::error::HandleStarknetRelayError;

#[derive(Clone)]
pub struct StarknetToCosmosRelay {
    pub fields: Arc<dyn HasCosmosToStarknetRelayFields>,
}

impl Deref for StarknetToCosmosRelay {
    type Target = CosmosToStarknetRelayFields;

    fn deref(&self) -> &Self::Target {
        &self.fields.fields()
    }
}

pub struct StarknetToCosmosRelayComponents;

impl HasComponents for StarknetToCosmosRelay {
    type Components = StarknetToCosmosRelayComponents;
}

impl<Name> DelegateComponent<Name> for StarknetToCosmosRelayComponents
where
    Self: IsDefaultRelayPreset<Name>,
{
    type Delegate = DefaultRelayPreset;
}

delegate_components! {
    StarknetToCosmosRelayComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetRelayError>,
        RuntimeTypeComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideHermesLogger,
        ChainTypeAtComponent<Index<0>>: WithType<CosmosChain>,
        ChainTypeAtComponent<Index<1>>: WithType<StarknetChain>,
        ChainGetterAtComponent<Index<0>>:
            UseField<symbol!("chain_a")>,
        ChainGetterAtComponent<Index<1>>:
            UseField<symbol!("chain_b")>,
        ClientIdAtGetterComponent<Index<0>, Index<1>>:
            UseField<symbol!("client_id_a")>,
        ClientIdAtGetterComponent<Index<1>, Index<0>>:
            UseField<symbol!("client_id_b")>,
        [
            ChainTypeAtComponent<Src>,
            ChainTypeAtComponent<Dst>,
            ChainGetterAtComponent<Src>,
            ChainGetterAtComponent<Dst>,
            ClientIdAtGetterComponent<Src, Dst>,
            ClientIdAtGetterComponent<Dst, Src>,
        ]:
            SelectRelayBToA,
    }
}

pub trait CanUseStarknetToCosmosRelay:
    Async
    + HasRelayChains<SrcChain = StarknetChain, DstChain = CosmosChain>
    + HasSourceTargetChainTypes
    + HasDestinationTargetChainTypes
    + HasTargetClientIds<SourceTarget>
    + HasTargetClientIds<DestinationTarget>
    + CanCreateClient<DestinationTarget>
    + CanCreateClient<SourceTarget>
    + CanRaiseRelayChainErrors
    + CanBuildTargetUpdateClientMessage<SourceTarget>
    + CanBuildTargetUpdateClientMessage<DestinationTarget>
    + CanSendTargetUpdateClientMessage<SourceTarget>
    + CanSendTargetUpdateClientMessage<DestinationTarget>
{
}

impl CanUseStarknetToCosmosRelay for StarknetToCosmosRelay {}
