use cgp::core::error::{DelegateErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
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
use hermes_relayer_components::relay::traits::chains::ProvideRelayChains;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::impls::error::HandleStarknetError;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

#[derive(Clone, HasField)]
pub struct StarknetToCosmosRelay {
    pub runtime: HermesRuntime,
    pub src_chain: CosmosChain,
    pub dst_chain: StarknetChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
}

pub struct StarknetToCosmosRelayComponents;

impl HasComponents for StarknetToCosmosRelay {
    type Components = StarknetToCosmosRelayComponents;
}

with_default_relay_components! {
    delegate_components! {
        StarknetToCosmosRelayComponents {
            @DefaultRelayComponents: DefaultRelayComponents,
        }
    }
}

delegate_components! {
    StarknetToCosmosRelayComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DelegateErrorRaiser<HandleStarknetError>,
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

impl ProvideRelayChains<StarknetToCosmosRelay> for StarknetToCosmosRelayComponents {
    type SrcChain = CosmosChain;

    type DstChain = CosmosChain;

    type Packet = Packet;

    fn src_chain(relay: &StarknetToCosmosRelay) -> &CosmosChain {
        &relay.src_chain
    }

    fn dst_chain(relay: &StarknetToCosmosRelay) -> &CosmosChain {
        &relay.src_chain
    }

    fn src_client_id(relay: &StarknetToCosmosRelay) -> &ClientId {
        &relay.src_client_id
    }

    fn dst_client_id(relay: &StarknetToCosmosRelay) -> &ClientId {
        &relay.dst_client_id
    }
}
