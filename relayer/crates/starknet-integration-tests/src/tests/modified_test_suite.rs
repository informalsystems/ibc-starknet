use cgp::core::field::Index;
use hermes_core::test_components::test_case::traits::test_case::TestCase;
use hermes_cosmos::error::types::Error;
use hermes_cosmos::integration_tests::init::init_test_runtime;
use hermes_ibc_test_suite::tests::client_refresh::TestRefreshClient;

use crate::utils::init_starknet_test_driver;

#[test]
fn test_client_refresh() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let test_driver = init_starknet_test_driver(&runtime).await?;

        <TestRefreshClient<Index<0>, Index<1>>>::default()
            .run_test(&test_driver)
            .await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
