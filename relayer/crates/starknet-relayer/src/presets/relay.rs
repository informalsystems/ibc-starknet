#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::core::error::{
        ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
    };
    use cgp::core::field::{Index, UseField};
    use cgp::prelude::*;
    use hermes_cosmos_relayer::contexts::chain::CosmosChain;
    use hermes_error::impls::UseHermesError;
    use hermes_logging_components::traits::logger::LoggerComponent;
    use hermes_relayer_components::components::default::relay::*;
    use hermes_relayer_components::multi::traits::chain_at::{
        ChainGetterAtComponent, ChainTypeProviderAtComponent,
    };
    use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
    use hermes_relayer_components::relay::impls::packet_lock::PacketMutexGetterComponent;
    use hermes_runtime::types::runtime::HermesRuntime;
    use hermes_runtime_components::traits::runtime::{
        RuntimeGetterComponent, RuntimeTypeProviderComponent,
    };
    use hermes_starknet_chain_context::contexts::chain::StarknetChain;
    use hermes_tracing_logging_components::contexts::logger::TracingLogger;
    use DefaultRelayPreset::re_exports::*;

    use crate::impls::error::HandleStarknetRelayError;

    DefaultRelayPreset::with_components! {
        | Components | {
            cgp_preset! {
                StarknetCommonRelayContextPreset {
                    [
                        ErrorTypeProviderComponent,
                        ErrorWrapperComponent,
                    ]: UseHermesError,
                    ErrorRaiserComponent: UseDelegate<HandleStarknetRelayError>,
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
                    ChainGetterAtComponent<Index<0>>:
                        UseField<symbol!("chain_a")>,
                    ChainGetterAtComponent<Index<1>>:
                        UseField<symbol!("chain_b")>,
                    ClientIdAtGetterComponent<Index<0>, Index<1>>:
                        UseField<symbol!("client_id_a")>,
                    ClientIdAtGetterComponent<Index<1>, Index<0>>:
                        UseField<symbol!("client_id_b")>,
                    PacketMutexGetterComponent:
                        UseField<symbol!("packet_lock_mutex")>,
                    Components: DefaultRelayPreset::Provider,
                }
            }
        }
    }
}
