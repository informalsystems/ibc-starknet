use core::marker::PhantomData;
use core::time::Duration;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use cgp::core::field::Index;
use cgp::extra::run::CanRun;
use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_chain_components::traits::queries::chain_status::{
    CanQueryChainHeight, CanQueryChainStatus,
};
use hermes_chain_components::traits::queries::channel_end::CanQueryChannelEnd;
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
use hermes_cosmos_test_components::chain::impls::transfer::amount::derive_ibc_denom;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_cosmos_test_components::chain::types::denom::{Denom as HermesDenom, Denom as IbcDenom};
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::Error;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_components::impls::subscription::CanCreateStarknetEventSubscription;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::impls::types::config::StarknetChainConfig;
use hermes_starknet_chain_components::impls::types::message::StarknetMessage;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::contract::invoke::CanInvokeContract;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::types::cosmos::height::Height;
use hermes_starknet_chain_components::types::messages::ibc::denom::{
    Denom, PrefixedDenom, TracePrefix,
};
use hermes_starknet_chain_components::types::messages::ibc::ibc_transfer::MsgTransfer;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_components::types::register::{MsgRegisterApp, MsgRegisterClient};
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_chain_context::contexts::encoding::event::StarknetEventEncoding;
use hermes_starknet_relayer::contexts::builder::StarknetBuilder;
use hermes_starknet_relayer::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use hex::FromHex;
use ibc::core::connection::types::version::Version as IbcConnectionVersion;
use ibc::core::host::types::identifiers::{
    ChannelId, ClientId, ConnectionId, PortId, PortId as IbcPortId,
};
use ibc::core::primitives::Timestamp;
use poseidon::Poseidon3Hasher;
use serde::{Deserialize, Serialize};
use starknet::accounts::{Call, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::types::{Felt, U256};
use starknet::macros::{selector, short_string};
use starknet::providers::Provider;
use starknet::signers::{LocalWallet, SigningKey};
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
pub struct TestnetDb {
    osmosis: OsmosisTestnet,
    starknet: StarknetTestnet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarknetTestnet {
    pub hash: HashMap<String, Felt>,
    pub address: HashMap<String, StarknetAddress>,
    pub client: Option<ClientId>,
    pub connection: Option<ConnectionId>,
    pub channel: Option<ChannelId>,
    // port id is "transfer"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsmosisTestnet {
    pub wasm_code_hash: String,
    pub client: Option<ClientId>,
    pub connection: Option<ConnectionId>,
    pub channel: Option<ChannelId>,
}

#[derive(Debug)]
pub struct ConfigDumper<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    pub path: String,
    pub config: T,
}

impl<T> Drop for ConfigDumper<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    fn drop(&mut self) {
        match toml::to_string_pretty(&self.config) {
            Ok(value_str) => match std::fs::write(&self.path, &value_str) {
                Ok(_) => info!("wrote config to file: {}", self.path),
                Err(e) => {
                    eprintln!("failed to write config to file: {:?}", e);
                    eprintln!("config: {}", value_str);
                }
            },
            Err(e) => eprintln!("failed to serialize: {:?}", e),
        }
    }
}

impl<T> ConfigDumper<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    pub fn new(path: &str) -> Result<Self, Error> {
        let config = toml::from_str(&std::fs::read_to_string(path)?)?;

        Ok(Self {
            path: path.to_string(),
            config,
        })
    }
}

impl<T> Deref for ConfigDumper<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl<T> DerefMut for ConfigDumper<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}

impl StarknetTestnet {
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
        }

        Ok(self.hash[key])
    }

    pub async fn get_address<T>(
        &mut self,
        key: &str,
        starknet_chain: &StarknetChain,
        data: T,
    ) -> Result<StarknetAddress, Error>
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
        }

        Ok(self.address[key])
    }
}

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

        let starknet_relayer_wallet = starknet_chain_config.relayer_wallet.clone();

        let starknet_builder = StarknetBuilder::new(
            CosmosBuilder::new(
                vec![cosmos_chain_config],
                runtime.clone(),
                Default::default(),
                Default::default(),
                Default::default(),
                [(
                    OSMOSIS_TESTNET_CHAIN_ID.parse()?,
                    osmosis_secp256k1_keypair.clone(),
                )]
                .into(),
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

        let mut starknet_contract_db = ConfigDumper::<TestnetDb>::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../starknet_db.toml"
        ))?;

        let ibc_core_address = starknet_contract_db
            .starknet
            .get_address("IBC_CORE", &starknet_chain, ())
            .await?;

        let comet_client_address = starknet_contract_db
            .starknet
            .get_address("COMET_CLIENT", &starknet_chain, ibc_core_address)
            .await?;

        starknet_chain.ibc_core_contract_address = Some(ibc_core_address);
        starknet_chain.ibc_client_contract_address = Some(comet_client_address);

        let erc20_class_hash = starknet_contract_db
            .starknet
            .get_hash("ERC20", &starknet_chain)
            .await?;

        let ics20_class_hash = starknet_contract_db
            .starknet
            .get_hash("ICS20", &starknet_chain)
            .await?;

        let comet_client_class_hash = starknet_contract_db
            .starknet
            .get_hash("COMET_CLIENT", &starknet_chain)
            .await?;

        let ics20_contract_address = starknet_contract_db
            .starknet
            .get_address(
                "ICS20",
                &starknet_chain,
                (ibc_core_address, erc20_class_hash),
            )
            .await?;

        starknet_chain.event_encoding = StarknetEventEncoding {
            erc20_hashes: [erc20_class_hash].into(),
            ics20_hashes: [ics20_class_hash].into(),
            ibc_client_hashes: [comet_client_class_hash].into(),
            ibc_core_contract_addresses: [ibc_core_address].into(),
        };

        let cairo_encoding = StarknetCairoEncoding;

        if starknet_contract_db.starknet.client.is_none() {
            {
                // register comet client contract with ibc-core

                info!("trying to register comet client contract with ibc-core");

                let register_client = MsgRegisterClient {
                    client_type: short_string!("07-tendermint"),
                    contract_address: comet_client_address,
                };

                let calldata = cairo_encoding.encode(&register_client)?;

                let call = Call {
                    to: *ibc_core_address,
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

            starknet_contract_db.starknet.client = Some(starknet_client_id);
        }

        if starknet_contract_db.osmosis.client.is_none() {
            let wasm_code_hash =
                <[u8; 32]>::from_hex(&starknet_contract_db.osmosis.wasm_code_hash)?;

            let cosmos_client_id = StarknetToCosmosRelay::create_client(
                DestinationTarget,
                &cosmos_chain,
                &starknet_chain,
                &StarknetCreateClientPayloadOptions { wasm_code_hash },
                &(),
            )
            .await?;

            info!("created client on Cosmos: {:?}", cosmos_client_id);

            starknet_contract_db.osmosis.client = Some(cosmos_client_id);
        }

        let starknet_client_id = starknet_contract_db.starknet.client.as_ref().unwrap();
        let cosmos_client_id = starknet_contract_db.osmosis.client.as_ref().unwrap();

        let client_state_on_starknet = starknet_chain
            .query_client_state_with_latest_height(PhantomData::<CosmosChain>, starknet_client_id)
            .await?;

        info!(
            "client state on Starknet: {} => {:?}",
            starknet_client_id, client_state_on_starknet
        );

        assert_eq!(&client_state_on_starknet.chain_id, cosmos_chain.chain_id());

        let client_state_on_cosmos = cosmos_chain
            .query_client_state_with_latest_height(PhantomData::<StarknetChain>, cosmos_client_id)
            .await?;

        info!(
            "client state on Cosmos: {} => {:?}",
            cosmos_client_id, client_state_on_cosmos
        );

        assert_eq!(
            &client_state_on_cosmos.client_state.chain_id,
            starknet_chain.chain_id()
        );

        starknet_chain.event_subscription =
            Some(starknet_chain.clone().create_starknet_event_subscription(
                starknet_chain.query_chain_height().await? - 5,
                ibc_core_address,
            ));

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

        if starknet_contract_db.starknet.connection.is_none()
            || starknet_contract_db.osmosis.connection.is_none()
        {
            let conn_init_option = CosmosInitConnectionOptions {
                delay_period: Duration::from_secs(0),
                connection_version: IbcConnectionVersion::compatibles().first().unwrap().clone(),
            };

            let (starknet_connection_id, cosmos_connection_id) = starknet_to_cosmos_relay
                .bootstrap_connection(&conn_init_option)
                .await?;

            info!(
                "created connection: {:?}(Starknet) <> {:?}(Osmosis)",
                starknet_connection_id, cosmos_connection_id
            );

            starknet_contract_db.starknet.connection = Some(starknet_connection_id);
            starknet_contract_db.osmosis.connection = Some(cosmos_connection_id);
        }

        let starknet_connection_id = starknet_contract_db.starknet.connection.as_ref().unwrap();
        let cosmos_connection_id = starknet_contract_db.osmosis.connection.as_ref().unwrap();

        let starknet_connection_end = CanQueryConnectionEnd::<CosmosChain>::query_connection_end(
            &starknet_chain,
            starknet_connection_id,
            &starknet_chain.query_chain_height().await?,
        )
        .await?;

        info!(
            "starknet_connection: {:?} => {:?}",
            starknet_connection_id, starknet_connection_end
        );

        let cosmos_connection_end = CanQueryConnectionEnd::<StarknetChain>::query_connection_end(
            &cosmos_chain,
            cosmos_connection_id,
            &cosmos_chain.query_chain_height().await?,
        )
        .await?;

        info!(
            "cosmos_connection: {:?} => {:?}",
            cosmos_connection_id, cosmos_connection_end
        );

        // channel handshake

        let ics20_port = IbcPortId::transfer();

        if starknet_contract_db.starknet.channel.is_none()
            || starknet_contract_db.osmosis.channel.is_none()
        {
            {
                // register the ICS20 contract with the IBC core contract

                let register_app = MsgRegisterApp {
                    port_id: ics20_port.clone(),
                    contract_address: ics20_contract_address,
                };

                let register_call_data = cairo_encoding.encode(&register_app)?;

                let call = Call {
                    to: *ibc_core_address,
                    selector: selector!("bind_port_id"),
                    calldata: register_call_data,
                };

                let message = StarknetMessage::new(call);

                let response = starknet_chain.send_message(message).await?;

                info!("register ics20 response: {:?}", response);
            }

            let init_channel_options =
                CosmosInitChannelOptions::new(starknet_connection_id.clone());

            let (starknet_channel_id, cosmos_channel_id) = starknet_to_cosmos_relay
                .bootstrap_channel(
                    &ics20_port.clone(),
                    &ics20_port.clone(),
                    &init_channel_options,
                )
                .await?;

            info!(
                "created channel: {:?}(Starknet) <> {:?}(Osmosis)",
                starknet_channel_id, cosmos_channel_id
            );

            starknet_contract_db.starknet.channel = Some(starknet_channel_id);
            starknet_contract_db.osmosis.channel = Some(cosmos_channel_id);
        }

        let starknet_channel_id = starknet_contract_db.starknet.channel.as_ref().unwrap();
        let cosmos_channel_id = starknet_contract_db.osmosis.channel.as_ref().unwrap();

        let starknet_channel_end = CanQueryChannelEnd::<CosmosChain>::query_channel_end(
            &starknet_chain,
            starknet_channel_id,
            &PortId::transfer(),
            &starknet_chain.query_chain_height().await?,
        )
        .await?;

        info!(
            "starknet_channel: {:?} => {:?}",
            starknet_channel_id, starknet_channel_end
        );

        let cosmos_channel_end = CanQueryChannelEnd::<StarknetChain>::query_channel_end(
            &cosmos_chain,
            cosmos_channel_id,
            &PortId::transfer(),
            &cosmos_chain.query_chain_height().await?,
        )
        .await?;

        info!(
            "cosmos_channel: {:?} => {:?}",
            cosmos_channel_id, cosmos_channel_end
        );

        // submit ics20 transfer to Cosmos

        let wallet_cosmos_a = CosmosTestWallet {
            id: "cosmos-relayer".into(),
            address: osmosis_secp256k1_keypair.account(),
            keypair: osmosis_secp256k1_keypair,
        };
        let address_cosmos_a = &wallet_cosmos_a.address;
        let wallet_starknet_b = &starknet_relayer_wallet;
        let address_starknet_b = &wallet_starknet_b.account_address;
        let transfer_quantity = 118u128;
        let denom_cosmos = HermesDenom::base("uosmo");

        let _starknet_account_b = SingleOwnerAccount::new(
            starknet_chain.rpc_client.clone(),
            LocalWallet::from_signing_key(SigningKey::from_secret_scalar(
                wallet_starknet_b.signing_key,
            )),
            *wallet_starknet_b.account_address,
            starknet_chain.rpc_client.chain_id().await?,
            ExecutionEncoding::New,
        );

        let balance_cosmos_a_step_0 = cosmos_chain
            .query_balance(address_cosmos_a, &denom_cosmos)
            .await?;

        info!(
            "cosmos balance before transfer: {}",
            balance_cosmos_a_step_0
        );

        let _packet = <CosmosChain as CanIbcTransferToken<StarknetChain>>::ibc_transfer_token(
            &cosmos_chain,
            cosmos_channel_id,
            &IbcPortId::transfer(),
            &wallet_cosmos_a,
            address_starknet_b,
            &Amount::new(transfer_quantity, denom_cosmos.clone()),
            &None,
        )
        .await?;

        // cosmos_to_starknet_relay.relay_packet(&packet).await?;

        let balance_cosmos_a_step_1 = cosmos_chain
            .query_balance(address_cosmos_a, &denom_cosmos)
            .await?;

        info!("cosmos balance after transfer: {}", balance_cosmos_a_step_1);

        // assert_eq!(
        //     balance_cosmos_a_step_0.quantity,
        //     balance_cosmos_a_step_1.quantity + transfer_quantity
        // );

        // Wait for background relayer to relay packet.
        // We cannot poll the balance here, because the IBC denom will only
        // be relayed after the first token transfer.
        runtime.sleep(Duration::from_secs(60)).await;

        let ics20_token_address: StarknetAddress = {
            let ibc_prefixed_denom = PrefixedDenom {
                trace_path: vec![TracePrefix {
                    port_id: ics20_port.to_string(),
                    channel_id: starknet_channel_id.to_string(),
                }],
                base: Denom::Hosted(denom_cosmos.to_string()),
            };

            let mut denom_serialized = vec![];

            {
                // https://github.com/informalsystems/ibc-starknet/blob/06cb7587557e6f3bef323abe7b5d9c3ab35bd97a/cairo-contracts/packages/apps/src/transfer/types.cairo#L120-L130
                for trace_prefix in &ibc_prefixed_denom.trace_path {
                    denom_serialized.extend(cairo_encoding.encode(trace_prefix)?);
                }

                denom_serialized.extend(cairo_encoding.encode(&ibc_prefixed_denom.base)?);
            }

            // https://github.com/informalsystems/ibc-starknet/blob/06cb7587557e6f3bef323abe7b5d9c3ab35bd97a/cairo-contracts/packages/utils/src/utils.cairo#L35
            let ibc_prefixed_denom_key = Poseidon3Hasher::digest(&denom_serialized);

            let calldata = cairo_encoding.encode(&product![ibc_prefixed_denom_key])?;

            let output = starknet_chain
                .call_contract(
                    &ics20_contract_address,
                    &selector!("ibc_token_address"),
                    &calldata,
                )
                .await?;

            cairo_encoding.decode(&output)?
        };

        info!("ics20 token address: {:?}", ics20_token_address);

        let balance_starknet_b_step_1 = starknet_chain
            .query_token_balance(&ics20_token_address, address_starknet_b)
            .await?;

        info!(
            "starknet balance after transfer: {}",
            balance_starknet_b_step_1
        );

        // assert_eq!(balance_starknet_b_step_1.quantity, transfer_quantity.into());

        // create ibc transfer message

        let starknet_ics20_send_message = {
            let current_starknet_time = starknet_chain.query_chain_status().await?.time;

            let denom = PrefixedDenom {
                trace_path: vec![TracePrefix {
                    port_id: ics20_port.to_string(),
                    channel_id: starknet_channel_id.to_string(),
                }],
                base: Denom::Hosted(denom_cosmos.to_string()),
            };

            MsgTransfer {
                port_id_on_a: ics20_port.clone(),
                chan_id_on_a: starknet_channel_id.clone(),
                denom,
                amount: transfer_quantity.into(),
                receiver: address_cosmos_a.clone(),
                memo: String::new(),
                timeout_height_on_b: Height {
                    revision_number: 0,
                    revision_height: 0,
                },
                timeout_timestamp_on_b: Timestamp::from_nanoseconds(
                    u64::try_from(current_starknet_time.unix_timestamp() + 1800).unwrap()
                        * 1_000_000_000,
                ),
            }
        };

        // submit to ics20 contract
        {
            let call_data = cairo_encoding.encode(&starknet_ics20_send_message)?;

            starknet_chain
                .invoke_contract(
                    &ics20_contract_address,
                    &selector!("send_transfer"),
                    &call_data,
                )
                .await?;
        };

        runtime.sleep(Duration::from_secs(60)).await;

        // cosmos_chain
        //     .assert_eventual_amount(address_cosmos_a, &balance_cosmos_a_step_0)
        //     .await?;

        let balance_starknet_b_step_2 = starknet_chain
            .query_token_balance(&ics20_token_address, address_starknet_b)
            .await?;

        info!(
            "starknet balance after transfer back: {}",
            balance_starknet_b_step_2
        );

        // assert_eq!(balance_starknet_b_step_2.quantity, 0u64.into());

        // send starknet erc20 token to cosmos

        let erc20_token_address = &STARKNET_STRK.into();

        info!("erc20 token address: {:?}", erc20_token_address);

        let balance_starknet_step_0 = starknet_chain
            .query_token_balance(erc20_token_address, address_starknet_b)
            .await?;

        info!("erc20 balance on starknet: {}", balance_starknet_step_0);

        {
            // approve ics20 contract to spend the tokens for address_starknet_b
            let call_data = cairo_encoding.encode(&product![
                ics20_contract_address,
                U256::from(transfer_quantity)
            ])?;

            starknet_chain
                .invoke_contract(erc20_token_address, &selector!("approve"), &call_data)
                .await?;
        }

        // submit ics20 transfer from Starknet to Cosmos

        let starknet_ics20_send_message = {
            let current_starknet_time = starknet_chain.query_chain_status().await?.time;

            let denom = PrefixedDenom {
                trace_path: vec![],
                base: Denom::Native(*erc20_token_address),
            };

            MsgTransfer {
                port_id_on_a: ics20_port.clone(),
                chan_id_on_a: starknet_channel_id.clone(),
                denom,
                amount: transfer_quantity.into(),
                receiver: address_cosmos_a.clone(),
                memo: String::new(),
                timeout_height_on_b: Height {
                    revision_number: 0,
                    revision_height: 0,
                },
                timeout_timestamp_on_b: Timestamp::from_nanoseconds(
                    u64::try_from(current_starknet_time.unix_timestamp() + 1800).unwrap()
                        * 1_000_000_000,
                ),
            }
        };

        {
            let call_data = cairo_encoding.encode(&starknet_ics20_send_message)?;

            starknet_chain
                .invoke_contract(
                    &ics20_contract_address,
                    &selector!("send_transfer"),
                    &call_data,
                )
                .await?;
        };

        let cosmos_ibc_denom = derive_ibc_denom(
            &ics20_port,
            cosmos_channel_id,
            &IbcDenom::base(&erc20_token_address.to_string()),
        )?;

        info!("cosmos ibc denom: {:?}", cosmos_ibc_denom);

        // cosmos_chain
        //     .assert_eventual_amount(
        //         address_cosmos_a,
        //         &Amount::new(transfer_quantity, cosmos_ibc_denom.clone()),
        //     )
        //     .await?;

        runtime.sleep(Duration::from_secs(60)).await;

        let balance_starknet_relayer_step_3 = starknet_chain
            .query_token_balance(erc20_token_address, address_starknet_b)
            .await?;

        info!(
            "starknet balance after transfer from starknet: {}",
            balance_starknet_relayer_step_3
        );

        // assert_eq!(
        //     balance_starknet_relayer_step_3.quantity,
        //     balance_starknet_step_0.quantity - transfer_quantity.into()
        // );

        // send the tokens back to starknet

        let _packet = <CosmosChain as CanIbcTransferToken<StarknetChain>>::ibc_transfer_token(
            &cosmos_chain,
            cosmos_channel_id,
            &IbcPortId::transfer(),
            &wallet_cosmos_a,
            address_starknet_b,
            &Amount::new(transfer_quantity, cosmos_ibc_denom.clone()),
            &None,
        )
        .await?;

        let balance_cosmos_a_step_4 = cosmos_chain
            .query_balance(address_cosmos_a, &cosmos_ibc_denom)
            .await?;

        info!(
            "cosmos balance after transfer back to starknet: {}",
            balance_cosmos_a_step_4
        );

        // assert_eq!(balance_cosmos_a_step_4.quantity, 0u64.into());

        // starknet_chain
        //     .assert_eventual_amount(address_starknet_b, &balance_starknet_step_0)
        //     .await?;

        runtime.sleep(Duration::from_secs(60)).await;

        Ok(())
    })
}
