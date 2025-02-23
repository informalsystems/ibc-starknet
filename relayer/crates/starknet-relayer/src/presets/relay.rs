#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
    use cgp::core::field::{Index, UseField, WithField};
    use cgp::core::types::WithType;
    use cgp::prelude::*;
    use hermes_cosmos_relayer::contexts::chain::CosmosChain;
    use hermes_error::impls::ProvideHermesError;
    use hermes_logger::UseHermesLogger;
    use hermes_logging_components::traits::has_logger::{
        GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeProviderComponent,
    };
    use hermes_relayer_components::components::default::relay::*;
    use hermes_relayer_components::multi::traits::chain_at::{
        ChainGetterAtComponent, ChainTypeAtComponent,
    };
    use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
    use hermes_relayer_components::relay::impls::packet_lock::PacketMutexGetterComponent;
    use hermes_runtime::types::runtime::HermesRuntime;
    use hermes_runtime_components::traits::runtime::{
        RuntimeGetterComponent, RuntimeTypeProviderComponent,
    };
    use hermes_starknet_chain_context::contexts::chain::StarknetChain;
    use DefaultRelayPreset::re_exports::*;

    use crate::impls::error::HandleStarknetRelayError;

    DefaultRelayPreset::with_components! {
        | Components | {
            cgp_preset! {
                StarknetCommonRelayContextPreset {
                    ErrorTypeProviderComponent: ProvideHermesError,
                    ErrorRaiserComponent: UseDelegate<HandleStarknetRelayError>,
                    RuntimeTypeProviderComponent: WithType<HermesRuntime>,
                    RuntimeGetterComponent: WithField<symbol!("runtime")>,
                    [
                        LoggerTypeProviderComponent,
                        LoggerGetterComponent,
                        GlobalLoggerGetterComponent,
                    ]:
                        UseHermesLogger,
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
                    PacketMutexGetterComponent:
                        UseField<symbol!("packet_lock_mutex")>,
                    Components: DefaultRelayPreset::Provider,
                }
            }
        }
    }
}
