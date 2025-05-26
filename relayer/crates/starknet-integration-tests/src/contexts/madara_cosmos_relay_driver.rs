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
    CosmosToMadaraRelay, MadaraCosmosBiRelay, MadaraToCosmosRelay,
};

#[cgp_context(MadaraCosmosRelayDriverComponents)]
#[derive(HasField)]
pub struct MadaraCosmosRelayDriver {
    pub birelay: MadaraCosmosBiRelay,
}

delegate_components! {
    MadaraCosmosRelayDriverComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        ChainTypeProviderAtComponent<Index<0>>:
            UseType<MadaraChain>,
        ChainTypeProviderAtComponent<Index<1>>:
            UseType<CosmosChain>,
        RelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<MadaraToCosmosRelay>,
        RelayTypeProviderAtComponent<Index<1>, Index<0>>:
            UseType<CosmosToMadaraRelay>,
        BiRelayTypeProviderAtComponent<Index<0>, Index<1>>:
            UseType<MadaraCosmosBiRelay>,
        BiRelayGetterAtComponent<Index<0>, Index<1>>:
            UseField<symbol!("birelay")>,
    }
}

#[cgp_provider(RelayerBackgroundRunnerComponent)]
impl RelayerBackgroundRunner<MadaraCosmosRelayDriver> for MadaraCosmosRelayDriverComponents {
    type RunHandle<'a> = AbortOnDrop;

    async fn run_relayer_in_background(
        relay_driver: &MadaraCosmosRelayDriver,
    ) -> Result<AbortOnDrop, Error> {
        let birelay = relay_driver.birelay.clone();

        let handle = tokio::spawn(async move {
            let _ = birelay.run().await;
        });

        Ok(AbortOnDrop(handle.abort_handle()))
    }
}
