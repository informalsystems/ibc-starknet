use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_error::types::Error;
use hermes_ibc_test_suite::tests::clearing::TestPacketClearing;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;
use hermes_test_components::test_case::traits::test_case::TestCase;

use crate::utils::init_starknet_test_driver;

#[test]
fn test_ibc_transfer() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let test_driver = init_starknet_test_driver(&runtime).await?;

        TestIbcTransfer.run_test(&test_driver).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

#[test]
fn test_packet_clearing() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let test_driver = init_starknet_test_driver(&runtime).await?;

        TestPacketClearing.run_test(&test_driver).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
