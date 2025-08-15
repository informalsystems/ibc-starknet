use cgp::core::field::Index;
use hermes_core::test_components::test_case::traits::test_case::TestCase;
use hermes_cosmos::error::types::Error;
use hermes_cosmos::integration_tests::init::init_test_runtime;
use hermes_ibc_test_suite::tests::clearing::TestPacketClearing;
use hermes_ibc_test_suite::tests::misebehaviour::TestMisbehaviourDetection;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;
use hermes_ibc_test_suite::tests::upgrade_client::TestUpgradeClient;

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

// Misbehaviour detection is split into 2 to avoid issues when forking Cosmos node
#[test]
fn test_misbehaviour_detection_0_to_1() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let test_driver = init_starknet_test_driver(&runtime).await?;

        <TestMisbehaviourDetection<Index<0>, Index<1>>>::default()
            .run_test(&test_driver)
            .await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

// Misbehaviour detection is split into 2 to avoid issues when forking Cosmos node
#[test]
fn test_misbehaviour_detection_1_to_0() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let test_driver = init_starknet_test_driver(&runtime).await?;

        <TestMisbehaviourDetection<Index<1>, Index<0>>>::default()
            .run_test(&test_driver)
            .await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

#[test]
fn test_upgrade_client() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let test_driver = init_starknet_test_driver(&runtime).await?;

        <TestUpgradeClient<Index<0>, Index<1>>>::default()
            .run_test(&test_driver)
            .await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
