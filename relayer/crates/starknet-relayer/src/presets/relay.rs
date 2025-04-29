#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::core::error::{
        ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
    };
    use cgp::core::field::{Index, UseField};
    use hermes_core::logging_components::traits::LoggerComponent;
    use hermes_core::relayer_components::components::default::DefaultRelayPreset;
    use hermes_core::relayer_components::multi::traits::chain_at::{
        ChainGetterAtComponent, ChainTypeProviderAtComponent,
    };
    use hermes_core::relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
    use hermes_core::relayer_components::relay::impls::PacketMutexGetterComponent;
    use hermes_core::runtime_components::traits::{
        RuntimeGetterComponent, RuntimeTypeProviderComponent,
    };
    use hermes_cosmos::error::impls::UseHermesError;
    use hermes_cosmos::relayer::contexts::CosmosChain;
    use hermes_cosmos::runtime::types::runtime::HermesRuntime;
    use hermes_cosmos::tracing_logging_components::contexts::TracingLogger;
    use hermes_starknet_chain_context::contexts::StarknetChain;
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
