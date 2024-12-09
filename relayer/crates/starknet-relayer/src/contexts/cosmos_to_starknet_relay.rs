use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::impls::ProvideHermesError;
use hermes_logger::ProvideHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::components::default::relay::*;
use hermes_relayer_components::error::impls::retry::ReturnMaxRetry;
use hermes_relayer_components::error::traits::retry::MaxErrorRetryGetterComponent;
use hermes_relayer_components::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::relay::traits::update_client_message_builder::{
    CanBuildTargetUpdateClientMessage, CanSendTargetUpdateClientMessage,
};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};
use hermes_starknet_chain_components::types::client_id::ClientId as StarknetClientId;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use ibc_relayer_types::core::ics24_host::identifier::ClientId as CosmosClientId;

use crate::impls::error::HandleStarknetRelayError;

#[derive(Clone, HasField)]
pub struct CosmosToStarknetRelay {
    pub runtime: HermesRuntime,
    pub src_chain: CosmosChain,
    pub dst_chain: StarknetChain,
    pub src_client_id: CosmosClientId,
    pub dst_client_id: StarknetClientId,
}

pub struct CosmosToStarknetRelayComponents;

impl HasComponents for CosmosToStarknetRelay {
    type Components = CosmosToStarknetRelayComponents;
}

with_default_relay_components! {
    delegate_components! {
        CosmosToStarknetRelayComponents {
            @DefaultRelayComponents: DefaultRelayComponents,
        }
    }
}

delegate_components! {
    CosmosToStarknetRelayComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetRelayError>,
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideHermesLogger,
        MaxErrorRetryGetterComponent:
            ReturnMaxRetry<3>,
    }
}

impl ProvideRelayChains<CosmosToStarknetRelay> for CosmosToStarknetRelayComponents {
    type SrcChain = CosmosChain;

    type DstChain = StarknetChain;

    fn src_chain(relay: &CosmosToStarknetRelay) -> &CosmosChain {
        &relay.src_chain
    }

    fn dst_chain(relay: &CosmosToStarknetRelay) -> &StarknetChain {
        &relay.dst_chain
    }

    fn src_client_id(relay: &CosmosToStarknetRelay) -> &CosmosClientId {
        &relay.src_client_id
    }

    fn dst_client_id(relay: &CosmosToStarknetRelay) -> &StarknetClientId {
        &relay.dst_client_id
    }
}

pub trait CanUseCosmosToStarknetRelay:
    HasRelayChains<SrcChain = CosmosChain, DstChain = StarknetChain>
    + CanCreateClient<DestinationTarget>
    + CanCreateClient<SourceTarget>
    + CanRaiseRelayChainErrors
    + CanBuildTargetUpdateClientMessage<SourceTarget>
    + CanBuildTargetUpdateClientMessage<DestinationTarget>
    + CanSendTargetUpdateClientMessage<SourceTarget>
    + CanSendTargetUpdateClientMessage<DestinationTarget>
{
}

impl CanUseCosmosToStarknetRelay for CosmosToStarknetRelay {}
