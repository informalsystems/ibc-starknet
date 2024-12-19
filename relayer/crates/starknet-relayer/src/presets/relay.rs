use cgp::core::component::UseDelegate;
pub use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::impls::use_field::{UseField, WithField};
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::impls::ProvideHermesError;
use hermes_logger::ProvideHermesLogger;
pub use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::components::default::relay::*;
pub use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeAtComponent,
};
pub use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_relayer_components::multi::types::index::Index;
use hermes_relayer_components::relay::impls::packet_lock::PacketMutexGetterComponent;
use hermes_runtime::types::runtime::HermesRuntime;
pub use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_starknet_chain_context::contexts::chain::StarknetChain;

use crate::impls::error::HandleStarknetRelayError;

with_default_relay_preset! {
    | Components | {
        cgp_preset! {
            StarknetCommonRelayContextPreset {
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
                PacketMutexGetterComponent:
                    UseField<symbol!("packet_lock_mutex")>,
                Components: DefaultRelayPreset,
            }
        }
    }
}
