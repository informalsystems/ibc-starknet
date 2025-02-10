use core::marker::PhantomData;
use core::time::Duration;
use std::collections::HashMap;

use cgp::core::field::Index;
use cgp::extra::run::CanRun;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainStatus;
use hermes_chain_components::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_chain_components::traits::queries::connection_end::CanQueryConnectionEnd;
use hermes_chain_components::traits::send_message::CanSendSingleMessage;
use hermes_chain_components::traits::types::chain_id::HasChainId;
use hermes_cosmos_chain_components::impls::types::config::CosmosChainConfig;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientOptions;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::Error;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_starknet_chain_components::impls::subscription::CanCreateStarknetEventSubscription;
use hermes_starknet_chain_components::impls::types::config::StarknetChainConfig;
use hermes_starknet_chain_components::impls::types::message::StarknetMessage;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_components::types::register::{MsgRegisterApp, MsgRegisterClient};
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_chain_context::contexts::encoding::event::StarknetEventEncoding;
use hermes_starknet_relayer::contexts::builder::StarknetBuilder;
use hermes_starknet_relayer::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hex::FromHex;
use ibc::core::connection::types::version::Version as IbcConnectionVersion;
use ibc::core::host::types::identifiers::PortId as IbcPortId;
use serde::{Deserialize, Serialize};
use starknet::accounts::Call;
use starknet::core::types::Felt;
use starknet::macros::{selector, short_string};
use tracing::info;

pub const COSMOS_HD_PATH: &str = "m/44'/118'/0'/0/0";

// https://github.com/osmosis-labs/testnets
pub const OSMOSIS_TESTNET_URL: &str = "rpc.testnet.osmosis.zone:443";
pub const OSMOSIS_TESTNET_CHAIN_ID: &str = "osmo-test-5";
pub const OSMOSIS_TOKEN: &str = "uosmo";

// https://docs.starknet.io/tools/fullnodes-rpc-providers/#open_endpoints
pub const STARKNET_TESTNET_URL: &str = "https://starknet-sepolia.public.blastapi.io/rpc/v0_7";
pub const STARKNET_TESTNET_CHAIN_ID: Felt = short_string!("SN_SEPOLIA");
// https://docs.starknet.io/chain-info
pub const STARKNET_STRK: Felt =
    Felt::from_hex_unchecked("0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d");
pub const STARKNET_ETH: Felt =
    Felt::from_hex_unchecked("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");

#[derive(Debug, Serialize, Deserialize)]
pub struct RelayerConfig {
    pub cosmos_chain_config: CosmosChainConfig,
    pub starknet_chain_config: StarknetChainConfig,
    pub mnemonic: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarknetContractDb {
    #[serde(skip)]
    pub path: String,
    pub hash: HashMap<String, Felt>,
    pub address: HashMap<String, Felt>,
}

impl StarknetContractDb {
    pub fn load(path: &str) -> Result<Self, Error> {
        let mut value: Self = toml::from_str(&std::fs::read_to_string(path)?)?;

        value.path = path.to_string();

        Ok(value)
    }

    pub fn write(&self) -> Result<(), Error> {
        Ok(std::fs::write(&self.path, toml::to_string_pretty(self)?)?)
    }

    pub async fn get_hash(
        &mut self,
        key: &str,
        starknet_chain: &StarknetChain,
    ) -> Result<Felt, Error> {
        if !self.hash.contains_key(key) {
            let contract_path = std::env::var(format!("{key}_CONTRACT"))?;

            let contract_str = std::fs::read_to_string(&contract_path)?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = starknet_chain
                .declare_contract(&contract)
                .await
                .map_err(|e| eyre::eyre!("failed to declare contract: {:?} {:?}", contract, e))?;

            info!("declared {} class: {:?}", key, class_hash);

            self.hash.insert(key.to_string(), class_hash);

            self.write()?;
        }

        Ok(self.hash[key])
    }

    pub async fn get_address<T>(
        &mut self,
        key: &str,
        starknet_chain: &StarknetChain,
        data: T,
    ) -> Result<Felt, Error>
    where
        StarknetCairoEncoding: CanEncode<ViaCairo, T>,
    {
        if !self.address.contains_key(key) {
            let class = self.get_hash(key, starknet_chain).await?;

            let call_data = StarknetCairoEncoding.encode(&data)?;

            let address = starknet_chain
                .deploy_contract(&class, false, &call_data)
                .await?;

            info!(
                "deployed {} contract : {:?} (class {:?})",
                key, address, class
            );

            self.address.insert(key.to_string(), address);

            self.write()?;
        }

        Ok(self.address[key])
    }
}

#[ignore]
#[test]
fn test_public_testnets() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        info!("Running public testnets tests");

        // StarknetRelayerConfig doesn't have mnenomic field
        let RelayerConfig {
            cosmos_chain_config,
            starknet_chain_config,
            mnemonic,
        } = {
            let config_string = std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../relayer.toml"
            ))?;

            toml::from_str(&config_string)?
        };

        let osmosis_secp256k1_keypair = Secp256k1KeyPair::from_mnemonic(
            &mnemonic,
            &COSMOS_HD_PATH.parse().expect("valid path"),
            &cosmos_chain_config.account_prefix,
        )
        .expect("valid key");

        info!(
            "osmosis relayer address: {}",
            osmosis_secp256k1_keypair.account()
        );
        info!(
            "starknet relayer address: {}",
            starknet_chain_config.relayer_wallet.account_address
        );

        let starknet_builder = StarknetBuilder::new(
            CosmosBuilder::new(
                vec![cosmos_chain_config],
                runtime.clone(),
                Default::default(),
                Default::default(),
                Default::default(),
                [(OSMOSIS_TESTNET_CHAIN_ID.parse()?, osmosis_secp256k1_keypair)].into(),
            ),
            runtime.clone(),
            starknet_chain_config,
        );

        let mut starknet_chain: StarknetChain = starknet_builder
            .build_chain(
                PhantomData::<Index<0>>,
                &STARKNET_TESTNET_CHAIN_ID.to_string().parse()?,
            )
            .await
            .map_err(|e| eyre::eyre!("failed to build starknet chain: {:?}", e))?;

        info!(
            "starknet_chain_status: {:?}",
            starknet_chain
                .query_chain_status()
                .await
                .map_err(|e| eyre::eyre!("failed to build starknet chain: {:?}", e))?
        );

        let cosmos_chain: CosmosChain = starknet_builder
            .build_chain(PhantomData::<Index<1>>, &OSMOSIS_TESTNET_CHAIN_ID.parse()?)
            .await?;

        info!(
            "cosmos_chain_status: {:?}",
            cosmos_chain.query_chain_status().await?
        );

        let wasm_code_hash = <[u8; 32]>::from_hex(
            "6be4d4cbb85ea2d7e0b17b7053e613af11e041617bdb163107dfd29f706318ef",
        )?;

        let mut starknet_contract_db = StarknetContractDb::load(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../starknet_db.toml"
        ))?;

        let ibc_core_address = starknet_contract_db
            .get_address("IBC_CORE", &starknet_chain, ())
            .await?;

        let comet_client_address = starknet_contract_db
            .get_address("COMET_CLIENT", &starknet_chain, ibc_core_address)
            .await?;

        starknet_chain.ibc_core_contract_address = Some(ibc_core_address);
        starknet_chain.ibc_client_contract_address = Some(comet_client_address);

        let erc20_class_hash = starknet_contract_db
            .get_hash("ERC20", &starknet_chain)
            .await?;

        let ics20_class_hash = starknet_contract_db
            .get_hash("ICS20", &starknet_chain)
            .await?;

        let comet_client_class_hash = starknet_contract_db
            .get_hash("COMET_CLIENT", &starknet_chain)
            .await?;

        starknet_chain.event_encoding = StarknetEventEncoding {
            erc20_hashes: [erc20_class_hash].into(),
            ics20_hashes: [ics20_class_hash].into(),
            ibc_client_hashes: [comet_client_class_hash].into(),
            ibc_core_contract_addresses: [ibc_core_address].into(),
        };

        let cairo_encoding = StarknetCairoEncoding;

        {
            // register comet client contract with ibc-core

            info!("trying to register comet client contract with ibc-core");

            let register_client = MsgRegisterClient {
                client_type: short_string!("07-tendermint"),
                contract_address: comet_client_address,
            };

            let calldata = cairo_encoding.encode(&register_client)?;

            let call = Call {
                to: ibc_core_address,
                selector: selector!("register_client"),
                calldata,
            };

            let message = StarknetMessage::new(call);

            let response = starknet_chain.send_message(message).await?;

            info!("IBC register client response: {:?}", response);
        }

        let starknet_client_id = StarknetToCosmosRelay::create_client(
            SourceTarget,
            &starknet_chain,
            &cosmos_chain,
            &CosmosCreateClientOptions {
                // unbonding period is 5 days on osmo-test-5
                trusting_period: Duration::from_secs(3 * 24 * 60 * 60),

                ..Default::default()
            },
            &(),
        )
        .await?;

        info!("created client on Starknet: {:?}", starknet_client_id);

        let cosmos_client_id = StarknetToCosmosRelay::create_client(
            DestinationTarget,
            &cosmos_chain,
            &starknet_chain,
            &StarknetCreateClientPayloadOptions { wasm_code_hash },
            &(),
        )
        .await?;

        info!("created client on Cosmos: {:?}", cosmos_client_id);

        let client_state_on_starknet = starknet_chain
            .query_client_state_with_latest_height(PhantomData::<CosmosChain>, &starknet_client_id)
            .await?;

        info!("client state on Starknet: {:?}", client_state_on_starknet);

        assert_eq!(&client_state_on_starknet.chain_id, cosmos_chain.chain_id());

        let client_state_on_cosmos = cosmos_chain
            .query_client_state_with_latest_height(PhantomData::<StarknetChain>, &cosmos_client_id)
            .await?;

        info!("client state on Cosmos: {:?}", client_state_on_cosmos);

        assert_eq!(
            &client_state_on_cosmos.client_state.chain_id,
            starknet_chain.chain_id()
        );

        let ics20_contract_address = starknet_contract_db
            .get_address("ICS20", &starknet_chain, ())
            .await?;

        starknet_chain.event_subscription = Some(
            starknet_chain
                .clone()
                .create_starknet_event_subscription(0, ibc_core_address),
        );

        let starknet_to_cosmos_relay = StarknetToCosmosRelay::new(
            runtime.clone(),
            starknet_chain.clone(),
            cosmos_chain.clone(),
            starknet_client_id.clone(),
            cosmos_client_id.clone(),
        );

        let cosmos_to_starknet_relay = CosmosToStarknetRelay::new(
            runtime.clone(),
            cosmos_chain.clone(),
            starknet_chain.clone(),
            cosmos_client_id.clone(),
            starknet_client_id.clone(),
        );

        {
            let starknet_to_cosmos_relay = starknet_to_cosmos_relay.clone();

            let cosmos_to_starknet_relay = cosmos_to_starknet_relay.clone();

            runtime.runtime.spawn(async move {
                let _ = starknet_to_cosmos_relay.run().await;
            });

            runtime.runtime.spawn(async move {
                let _ = cosmos_to_starknet_relay.run().await;
            });
        }

        // connection handshake

        let conn_init_option = CosmosInitConnectionOptions {
            delay_period: Duration::from_secs(0),
            connection_version: IbcConnectionVersion::compatibles().first().unwrap().clone(),
        };

        let (starknet_connection_id, cosmos_connection_id) = starknet_to_cosmos_relay
            .bootstrap_connection(&conn_init_option)
            .await?;

        info!("starknet_connection_id: {:?}", starknet_connection_id);
        info!("cosmos_connection_id: {:?}", cosmos_connection_id);

        let starknet_connection_end = CanQueryConnectionEnd::<CosmosChain>::query_connection_end(
            &starknet_chain,
            &starknet_connection_id,
            &starknet_chain.query_chain_height().await?,
        )
        .await?;

        info!(
            "starknet_connection: {:?} => {:?}",
            starknet_connection_id, starknet_connection_end
        );

        let cosmos_connection_end = CanQueryConnectionEnd::<StarknetChain>::query_connection_end(
            &cosmos_chain,
            &cosmos_connection_id,
            &cosmos_chain.query_chain_height().await?,
        )
        .await?;

        info!(
            "cosmos_connection: {:?} => {:?}",
            cosmos_connection_id, cosmos_connection_end
        );

        // channel handshake

        let ics20_port = IbcPortId::transfer();

        {
            // register the ICS20 contract with the IBC core contract

            let register_app = MsgRegisterApp {
                port_id: ics20_port.clone(),
                contract_address: ics20_contract_address,
            };

            let register_call_data = cairo_encoding.encode(&register_app)?;

            let call = Call {
                to: ibc_core_address,
                selector: selector!("bind_port_id"),
                calldata: register_call_data,
            };

            let message = StarknetMessage::new(call);

            let response = starknet_chain.send_message(message).await?;

            info!("register ics20 response: {:?}", response);
        }

        let init_channel_options = CosmosInitChannelOptions::new(starknet_connection_id);

        let (starknet_channel_id, cosmos_channel_id) = starknet_to_cosmos_relay
            .bootstrap_channel(
                &ics20_port.clone(),
                &ics20_port.clone(),
                &init_channel_options,
            )
            .await?;

        info!("starknet_channel_id: {:?}", starknet_channel_id);
        info!("cosmos_channel_id: {:?}", cosmos_channel_id);

        Ok(())
    })
}
