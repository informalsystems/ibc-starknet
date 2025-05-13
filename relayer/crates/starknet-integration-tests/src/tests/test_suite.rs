use cgp::core::field::Index;
use hermes_core::test_components::test_case::traits::test_case::TestCase;
use hermes_cosmos::error::types::Error;
use hermes_cosmos::integration_tests::init::init_test_runtime;
use hermes_ibc_test_suite::tests::clearing::TestPacketClearing;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;

use crate::utils::init_starknet_test_driver;

#[test]
fn test_ibc_transfer() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let test_driver = init_starknet_test_driver(&runtime).await?;

        <TestIbcTransfer<Index<0>, Index<1>>>::default()
            .run_test(&test_driver)
            .await?;

        <TestIbcTransfer<Index<1>, Index<0>>>::default()
            .run_test(&test_driver)
            .await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

#[test]
fn test_packet_clearing() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let test_driver = init_starknet_test_driver(&runtime).await?;

        <TestPacketClearing<Index<0>, Index<1>>>::default()
            .run_test(&test_driver)
            .await?;

        <TestPacketClearing<Index<1>, Index<0>>>::default()
            .run_test(&test_driver)
            .await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
