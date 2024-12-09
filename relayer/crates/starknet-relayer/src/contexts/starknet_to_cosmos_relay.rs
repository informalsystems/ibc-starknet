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
use hermes_relayer_components::components::default::relay::*;
use hermes_relayer_components::error::impls::retry::ReturnMaxRetry;
use hermes_relayer_components::error::traits::retry::MaxErrorRetryGetterComponent;
use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeAtComponent, HasChainTypeAt,
};
use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::traits::chains::CanRaiseRelayChainErrors;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::relay::traits::update_client_message_builder::{
    CanBuildTargetUpdateClientMessage, CanSendTargetUpdateClientMessage,
};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};
use hermes_starknet_chain_components::types::client_id::ClientId as StarknetClientId;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use ibc::core::host::types::identifiers::ClientId as CosmosClientId;

use crate::impls::error::HandleStarknetRelayError;

#[derive(Clone, HasField)]
pub struct StarknetToCosmosRelay {
    pub runtime: HermesRuntime,
    pub src_chain: StarknetChain,
    pub dst_chain: CosmosChain,
    pub src_client_id: StarknetClientId,
    pub dst_client_id: CosmosClientId,
}

pub struct StarknetToCosmosRelayComponents;

impl HasComponents for StarknetToCosmosRelay {
    type Components = StarknetToCosmosRelayComponents;
}

with_default_relay_components! {
    | Components | {
        delegate_components! {
            StarknetToCosmosRelayComponents {
                Components: DefaultRelayComponents,
            }
        }
    }
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
        ChainTypeAtComponent<Src>: WithType<StarknetChain>,
        ChainTypeAtComponent<Dst>: WithType<CosmosChain>,
        ChainGetterAtComponent<Src>:
            UseField<symbol!("src_chain")>,
        ChainGetterAtComponent<Dst>:
            UseField<symbol!("dst_chain")>,
        ClientIdAtGetterComponent<Src, Dst>:
            UseField<symbol!("src_client_id")>,
        ClientIdAtGetterComponent<Dst, Src>:
            UseField<symbol!("dst_client_id")>,
        MaxErrorRetryGetterComponent:
            ReturnMaxRetry<3>,
    }
}

pub trait CanUseStarknetToCosmosRelay:
    Async
    + HasChainTypeAt<Src, Chain = StarknetChain>
    + HasChainTypeAt<Dst, Chain = CosmosChain>
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
