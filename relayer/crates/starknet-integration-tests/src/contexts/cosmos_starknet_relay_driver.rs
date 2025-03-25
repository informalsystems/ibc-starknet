use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::Index;
use cgp::extra::run::CanRun;
use cgp::prelude::*;
use hermes_cosmos_integration_tests::contexts::relay_driver::AbortOnDrop;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_error::Error;
use hermes_relayer_components::multi::traits::birelay_at::{
    BiRelayGetterAtComponent, BiRelayTypeProviderAtComponent,
};
use hermes_relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayTypeProviderAtComponent;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_relayer::contexts::cosmos_starknet_birelay::CosmosStarknetBiRelay;
use hermes_starknet_relayer::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::relay_driver::run::{
    RelayerBackgroundRunner, RelayerBackgroundRunnerComponent,
};

#[cgp_context(StarknetRelayDriverComponents)]
#[derive(HasField)]
pub struct CosmosStarknetRelayDriver {
    pub birelay: CosmosStarknetBiRelay,
}

delegate_components! {
    StarknetRelayDriverComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        ChainTypeProviderAtComponent<Index<0>>:
            UseType<CosmosChain>,
        ChainTypeProviderAtComponent<Index<1>>:
            UseType<StarknetChain>,
        RelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<CosmosToStarknetRelay>,
        RelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<StarknetToCosmosRelay>,
        BiRelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<CosmosStarknetBiRelay>,
        BiRelayGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("birelay")>,
    }
}

#[cgp_provider(RelayerBackgroundRunnerComponent)]
impl RelayerBackgroundRunner<CosmosStarknetRelayDriver> for StarknetRelayDriverComponents {
    type RunHandle<'a> = AbortOnDrop;

    async fn run_relayer_in_background(
        relay_driver: &CosmosStarknetRelayDriver,
    ) -> Result<AbortOnDrop, Error> {
        let birelay = relay_driver.birelay.clone();

        let handle = tokio::spawn(async move {
            let _ = birelay.run().await;
        });

        Ok(AbortOnDrop(handle.abort_handle()))
    }
}
