use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::Index;
use cgp::extra::run::CanRun;
use cgp::prelude::*;
use hermes_core::relayer_components::multi::traits::birelay_at::{
    BiRelayGetterAtComponent, BiRelayTypeProviderAtComponent,
};
use hermes_core::relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::relay_at::RelayTypeProviderAtComponent;
use hermes_core::test_components::relay_driver::run::{
    RelayerBackgroundRunner, RelayerBackgroundRunnerComponent,
};
use hermes_cosmos::error::handlers::DebugError;
use hermes_cosmos::error::impls::UseHermesError;
use hermes_cosmos::error::Error;
use hermes_cosmos::integration_tests::contexts::AbortOnDrop;
use hermes_cosmos::relayer::contexts::CosmosChain;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_relayer::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use hermes_starknet_relayer::contexts::starknet_cosmos_birelay::StarknetCosmosBiRelay;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;

#[cgp_context(StarknetRelayDriverComponents)]
#[derive(HasField)]
pub struct StarknetCosmosRelayDriver {
    pub birelay: StarknetCosmosBiRelay,
}

delegate_components! {
    StarknetRelayDriverComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        ChainTypeProviderAtComponent<Index<0>>:
            UseType<StarknetChain>,
        ChainTypeProviderAtComponent<Index<1>>:
            UseType<CosmosChain>,
        RelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<StarknetToCosmosRelay>,
        RelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosToStarknetRelay>,
        BiRelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<StarknetCosmosBiRelay>,
        BiRelayGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("birelay")>,
    }
}

#[cgp_provider(RelayerBackgroundRunnerComponent)]
impl RelayerBackgroundRunner<StarknetCosmosRelayDriver> for StarknetRelayDriverComponents {
    type RunHandle<'a> = AbortOnDrop;

    async fn run_relayer_in_background(
        relay_driver: &StarknetCosmosRelayDriver,
    ) -> Result<AbortOnDrop, Error> {
        let birelay = relay_driver.birelay.clone();

        let handle = tokio::spawn(async move {
            let _ = birelay.run().await;
        });

        Ok(AbortOnDrop(handle.abort_handle()))
    }
}
