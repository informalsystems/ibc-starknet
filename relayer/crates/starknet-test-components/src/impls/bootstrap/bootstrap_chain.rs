use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr};

use cgp::core::error::CanRaiseAsyncError;
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
use hermes_prelude::*;
use hermes_starknet_chain_components::types::StarknetWallet;
use starknet::core::types::Felt;
use starknet::macros::felt;

use crate::types::{StarknetGenesisConfig, StarknetNodeConfig};

#[cgp_new_provider(ChainBootstrapperComponent)]
impl<Bootstrap, Runtime> ChainBootstrapper<Bootstrap> for BootstrapStarknet
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
        + CanGenerateRandom<u128>
        + CanCreateDir,
    Bootstrap::Chain: HasWalletType<Wallet = StarknetWallet>,
{
    async fn bootstrap_chain(
        bootstrap: &Bootstrap,
        chain_id_prefix: &str,
    ) -> Result<Bootstrap::ChainDriver, Bootstrap::Error> {
        let runtime = bootstrap.runtime();

        let postfix = <Runtime as CanGenerateRandom<u32>>::generate_random(runtime).await;

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

        // Arbitrarily chosen only for testing.
        let sequencer_private_key = {
            let low: Felt = <Runtime as CanGenerateRandom<u128>>::generate_random(runtime)
                .await
                .into();
            let high: Felt = <Runtime as CanGenerateRandom<u128>>::generate_random(runtime)
                .await
                .into();

            (high * felt!("0x100000000000000000000000000000000")) + low
        };

        let node_config = StarknetNodeConfig {
            rpc_addr,
            rpc_port,
            sequencer_private_key,
        };

        let chain_process = bootstrap
            .start_chain_full_nodes(&chain_home_dir, &node_config, &genesis_config)
            .await?;

        // For now, we hard code the wallets generated from madara
        // TODO: Can we retrieve the information from stdout.log?
        let wallets = BTreeMap::from([
            (
                "relayer".into(),
                StarknetWallet::from_signing_key(
                    felt!("0x055be462e718c4166d656d11f89e341115b8bc82389c3762a10eade04fcb225d"),
                    felt!("0x077e56c6dc32d40a67f6f7e6625c8dc5e570abe49c0a24e9202e4ae906abcc07"),
                ),
            ),
            (
                "user-a".into(),
                StarknetWallet::from_signing_key(
                    felt!("0x008a1719e7ca19f3d91e8ef50a48fc456575f645497a1d55f30e3781f786afe4"),
                    felt!("0x0514977443078cf1e0c36bc88b89ada9a46061a5cf728f40274caea21d76f174"),
                ),
            ),
            (
                "user-b".into(),
                StarknetWallet::from_signing_key(
                    felt!("0x0733a8e2bcced14dcc2608462bd96524fb64eef061689b6d976708efc2c8ddfd"),
                    felt!("0x00177100ae65c71074126963e695e17adf5b360146f960378b5cdfd9ed69870b"),
                ),
            ),
            (
                "relayer-2".into(),
                StarknetWallet::from_signing_key(
                    felt!("0x025073e0772b1e348a5da66ea67fb46f75ecdca1bd24dbbc98567cbf4a0e00b3"),
                    felt!("0x07ae55c8093920562c1cbab9edeb4eb52f788b93cac1d5721bda20c96100d743"),
                ),
            ),
        ]);

        let chain_driver = bootstrap
            .build_chain_driver(genesis_config, node_config, wallets, chain_process)
            .await?;

        Ok(chain_driver)
    }
}
