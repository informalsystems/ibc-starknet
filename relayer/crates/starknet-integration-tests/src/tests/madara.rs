use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_error::Error;

use crate::contexts::madara_bootstrap::MadaraBootstrap;
use crate::utils::init_starknet_bootstrap;

#[test]
fn test_madara_bootstrap() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let starknet_bootstrap = init_starknet_bootstrap(&runtime).await?;

        let _madara_bootstrap = MadaraBootstrap {
            fields: starknet_bootstrap.fields,
        };

        Ok(())
    })
}
