use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::Index;
use cgp::extra::run::CanRun;
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
use hermes_prelude::*;
use hermes_starknet_madara_tests::contexts::MadaraChain;
use hermes_starknet_relayer::contexts::{
    CosmosMadaraBiRelay, CosmosToMadaraRelay, MadaraToCosmosRelay,
};

#[cgp_context(CosmosMadaraRelayDriverComponents)]
#[derive(HasField)]
pub struct CosmosMadaraRelayDriver {
    pub birelay: CosmosMadaraBiRelay,
}

delegate_components! {
    CosmosMadaraRelayDriverComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        ChainTypeProviderAtComponent<Index<0>>:
            UseType<CosmosChain>,
        ChainTypeProviderAtComponent<Index<1>>:
            UseType<MadaraChain>,
        RelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<CosmosToMadaraRelay>,
        RelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<MadaraToCosmosRelay>,
        BiRelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<CosmosMadaraBiRelay>,
        BiRelayGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("birelay")>,
    }
}

#[cgp_provider(RelayerBackgroundRunnerComponent)]
impl RelayerBackgroundRunner<CosmosMadaraRelayDriver> for CosmosMadaraRelayDriverComponents {
    type RunHandle<'a> = AbortOnDrop;

    async fn run_relayer_in_background(
        relay_driver: &CosmosMadaraRelayDriver,
    ) -> Result<AbortOnDrop, Error> {
        let birelay = relay_driver.birelay.clone();

        let handle = tokio::spawn(async move {
            let _ = birelay.run().await;
        });

        Ok(AbortOnDrop(handle.abort_handle()))
    }
}
