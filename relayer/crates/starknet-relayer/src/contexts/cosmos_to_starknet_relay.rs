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
use hermes_relayer_components::error::impls::retry::ReturnMaxRetry;
use hermes_relayer_components::error::traits::retry::MaxErrorRetryGetterComponent;
use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeAtComponent,
};
use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_relayer_components::multi::types::index::Index;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::impls::selector::SelectRelayAToB;
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
use hermes_starknet_chain_components::types::client_id::ClientId as StarknetClientId;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use ibc::core::host::types::identifiers::ClientId as CosmosClientId;

use crate::impls::error::HandleStarknetRelayError;

#[derive(Clone)]
pub struct CosmosToStarknetRelay {
    pub fields: Arc<dyn HasCosmosToStarknetRelayFields>,
}

#[derive(HasField)]
pub struct CosmosToStarknetRelayFields {
    pub runtime: HermesRuntime,
    pub chain_a: CosmosChain,
    pub chain_b: StarknetChain,
    pub client_id_a: CosmosClientId,
    pub client_id_b: StarknetClientId,
}

pub trait HasCosmosToStarknetRelayFields: Async {
    fn fields(&self) -> &CosmosToStarknetRelayFields;
}

impl HasCosmosToStarknetRelayFields for CosmosToStarknetRelayFields {
    fn fields(&self) -> &CosmosToStarknetRelayFields {
        self
    }
}

impl Deref for CosmosToStarknetRelay {
    type Target = CosmosToStarknetRelayFields;

    fn deref(&self) -> &Self::Target {
        &self.fields.fields()
    }
}

pub struct CosmosToStarknetRelayComponents;

impl HasComponents for CosmosToStarknetRelay {
    type Components = CosmosToStarknetRelayComponents;
}

impl<Name> DelegateComponent<Name> for CosmosToStarknetRelayComponents
where
    Self: IsDefaultRelayPreset<Name>,
{
    type Delegate = DefaultRelayPreset;
}

delegate_components! {
    CosmosToStarknetRelayComponents {
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
        MaxErrorRetryGetterComponent:
            ReturnMaxRetry<3>,
        ChainTypeAtComponent<Index<0>>: WithType<CosmosChain>,
        ChainTypeAtComponent<Index<1>>: WithType<StarknetChain>,
        ChainGetterAtComponent<Index<0>>:
            UseField<symbol!("chain_a")>,
        ChainGetterAtComponent<Index<1>>:
            UseField<symbol!("chain_b")>,
        ClientIdAtGetterComponent<Src, Dst>:
            UseField<symbol!("client_id_a")>,
        ClientIdAtGetterComponent<Dst, Src>:
            UseField<symbol!("client_id_b")>,
        [
            ChainTypeAtComponent<Src>,
            ChainTypeAtComponent<Dst>,
            ChainGetterAtComponent<Src>,
            ChainGetterAtComponent<Dst>,
        ]:
            SelectRelayAToB,
    }
}

pub trait CanUseCosmosToStarknetRelay:
    Async
    + HasRelayChains<SrcChain = CosmosChain, DstChain = StarknetChain>
    + CanRaiseRelayChainErrors
    + HasSourceTargetChainTypes
    + HasDestinationTargetChainTypes
    + HasTargetClientIds<SourceTarget>
    + HasTargetClientIds<DestinationTarget>
    + CanCreateClient<DestinationTarget>
    + CanCreateClient<SourceTarget>
    + CanBuildTargetUpdateClientMessage<SourceTarget>
    + CanBuildTargetUpdateClientMessage<DestinationTarget>
    + CanSendTargetUpdateClientMessage<SourceTarget>
    + CanSendTargetUpdateClientMessage<DestinationTarget>
{
}

impl CanUseCosmosToStarknetRelay for CosmosToStarknetRelay {}
