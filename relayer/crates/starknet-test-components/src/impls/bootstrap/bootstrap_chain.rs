use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr};

use cgp::core::error::CanRaiseAsyncError;
use cgp::prelude::*;
use hermes_core::runtime_components::traits::{
    CanCreateDir, CanGenerateRandom, CanReserveTcpPort, HasChildProcessType, HasFilePathType,
    HasRuntime,
};
use hermes_core::test_components::bootstrap::traits::{
    ChainBootstrapper, ChainBootstrapperComponent,
};
use hermes_core::test_components::chain::traits::HasWalletType;
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_cosmos_core::test_components::bootstrap::traits::{
    CanBuildChainDriver, CanStartChainFullNodes, HasChainGenesisConfigType, HasChainNodeConfigType,
    HasChainStoreDir,
};
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use starknet::macros::felt;

use crate::types::genesis_config::StarknetGenesisConfig;
use crate::types::node_config::StarknetNodeConfig;

#[cgp_new_provider(ChainBootstrapperComponent)]
impl<Bootstrap, Runtime> ChainBootstrapper<Bootstrap> for BootstrapStarknetDevnet
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType
        + HasChainNodeConfigType<ChainNodeConfig = StarknetNodeConfig>
        + HasChainGenesisConfigType<ChainGenesisConfig = StarknetGenesisConfig>
        + CanBuildChainDriver
        + CanStartChainFullNodes
        + HasChainStoreDir
        + CanRaiseAsyncError<Runtime::Error>,
    Runtime: HasChildProcessType
        + CanReserveTcpPort
        + HasFilePathType
        + CanGenerateRandom<u32>
        + CanCreateDir,
    Bootstrap::Chain: HasWalletType<Wallet = StarknetWallet>,
{
    async fn bootstrap_chain(
        bootstrap: &Bootstrap,
        chain_id_prefix: &str,
    ) -> Result<Bootstrap::ChainDriver, Bootstrap::Error> {
        let runtime = bootstrap.runtime();

        let postfix = runtime.generate_random().await;

        let chain_home_dir = Runtime::join_file_path(
            bootstrap.chain_store_dir(),
            &Runtime::file_path_from_string(&format!("{chain_id_prefix}-{postfix}")),
        );

        runtime
            .create_dir(&chain_home_dir)
            .await
            .map_err(Bootstrap::raise_error)?;

        // FIXME: RPC address is set to localhost and port is set to a random free port
        // The values should be configurable to connect to a specific node
        let rpc_addr = IpAddr::V4(Ipv4Addr::LOCALHOST);

        let rpc_port = runtime
            .reserve_tcp_port()
            .await
            .map_err(Bootstrap::raise_error)?;

        // Use a hard-coded seed 0 for now
        let genesis_config = StarknetGenesisConfig {
            seed: 0,
            transfer_denom: felt!(
                "0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7"
            )
            .into(),
            staking_denom: felt!(
                "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d"
            )
            .into(),
        };

        let node_config = StarknetNodeConfig { rpc_addr, rpc_port };

        let chain_process = bootstrap
            .start_chain_full_nodes(&chain_home_dir, &node_config, &genesis_config)
            .await?;

        // For now, we hard code the wallets generated from devnet-rs
        let wallets = BTreeMap::from([
            (
                "relayer".into(),
                StarknetWallet {
                    account_address: felt!(
                        "0x64b48806902a367c8598f4f95c305e8c1a1acba5f082d294a43793113115691"
                    )
                    .into(),
                    signing_key: felt!("0x71d7bb07b9a64f6f78ac4c816aff4da9"),
                    public_key: felt!(
                        "0x39d9e6ce352ad4530a0ef5d5a18fd3303c3606a7fa6ac5b620020ad681cc33b"
                    ),
                },
            ),
            (
                "user-a".into(),
                StarknetWallet {
                    account_address: felt!(
                        "0x78662e7352d062084b0010068b99288486c2d8b914f6e2a55ce945f8792c8b1"
                    )
                    .into(),
                    signing_key: felt!("0xe1406455b7d66b1690803be066cbe5e"),
                    public_key: felt!(
                        "0x7a1bb2744a7dd29bffd44341dbd78008adb4bc11733601e7eddff322ada9cb"
                    ),
                },
            ),
            (
                "user-b".into(),
                StarknetWallet {
                    account_address: felt!(
                        "0x49dfb8ce986e21d354ac93ea65e6a11f639c1934ea253e5ff14ca62eca0f38e"
                    )
                    .into(),
                    signing_key: felt!("0xa20a02f0ac53692d144b20cb371a60d7"),
                    public_key: felt!(
                        "0xb8fd4ddd415902d96f61b7ad201022d495997c2dff8eb9e0eb86253e30fabc"
                    ),
                },
            ),
        ]);

        let chain_driver = bootstrap
            .build_chain_driver(genesis_config, node_config, wallets, chain_process)
            .await?;

        Ok(chain_driver)
    }
}
