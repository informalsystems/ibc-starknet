use core::fmt::Debug;
use core::hash::Hash;
use std::collections::{BTreeMap, HashSet};
use std::sync::OnceLock;

use cgp::extra::runtime::HasRuntimeType;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_core::chain_components::traits::{CanSendSingleMessage, HasMessageType};
use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_core::encoding_components::traits::{CanEncode, HasEncodedType, HasEncoding};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelInfo;
use hermes_core::runtime_components::traits::{ChildProcessOf, HasChildProcessType};
use hermes_core::test_components::chain::traits::HasWalletType;
use hermes_core::test_components::chain_driver::traits::{HasChain, HasChainType};
use hermes_core::test_components::driver::traits::HasChainDriverType;
use hermes_cosmos_core::test_components::bootstrap::traits::{
    ChainDriverBuilder, ChainDriverBuilderComponent, HasChainGenesisConfigType,
    HasChainNodeConfigType,
};
use hermes_prelude::*;
use hermes_starknet_chain_components::impls::{StarknetAddress, StarknetMessage};
use hermes_starknet_chain_components::traits::{
    CanDeclareContract, CanDeployContract, ContractClassOf, HasBlobType, HasContractClassHashType,
    HasContractClassType,
};
use hermes_starknet_chain_components::types::{MsgRegisterApp, MsgRegisterClient, PortId};
use starknet::core::types::Felt;
use starknet::macros::{selector, short_string};

use crate::traits::{CanDeployIbcContracts, IbcContractsDeployer, IbcContractsDeployerComponent};

#[cgp_auto_getter]
pub trait HasIbcContracts: HasChainType<Chain: HasContractClassType> {
    fn erc20_contract(&self) -> &ContractClassOf<Self::Chain>;

    fn ics20_contract(&self) -> &ContractClassOf<Self::Chain>;

    fn ibc_core_contract(&self) -> &ContractClassOf<Self::Chain>;

    fn comet_client_contract(&self) -> &ContractClassOf<Self::Chain>;

    fn comet_lib_contract(&self) -> &ContractClassOf<Self::Chain>;

    fn ics23_lib_contract(&self) -> &ContractClassOf<Self::Chain>;

    fn protobuf_lib_contract(&self) -> &ContractClassOf<Self::Chain>;
}

#[cgp_auto_getter]
pub trait HasChainContractFields: HasAddressType {
    fn ibc_core_contract_address(&self) -> &OnceLock<Self::Address>;

    fn ibc_client_contract_address(&self) -> &OnceLock<Self::Address>;

    fn ibc_ics20_contract_address(&self) -> &OnceLock<Self::Address>;
}

#[cgp_auto_getter]
pub trait HasEventContractFields<Chain>
where
    Chain: HasContractClassHashType + HasAddressType,
{
    fn erc20_hashes(&self) -> &OnceLock<HashSet<Chain::ContractClassHash>>;

    fn ics20_hashes(&self) -> &OnceLock<HashSet<Chain::ContractClassHash>>;

    fn ibc_client_hashes(&self) -> &OnceLock<HashSet<Chain::ContractClassHash>>;

    fn ibc_core_contract_addresses(&self) -> &OnceLock<HashSet<Chain::Address>>;
}

#[cgp_new_provider(IbcContractsDeployerComponent)]
impl<Bootstrap, Chain, CairoEncoding, EventEncoding> IbcContractsDeployer<Bootstrap>
    for DeployIbcContract
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasIbcContracts
        + CanLog<LevelInfo>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<Chain::Error>
        + CanRaiseAsyncError<CairoEncoding::Error>,
    Chain: HasEncoding<AsFelt, Encoding = CairoEncoding>
        + HasEncoding<AsStarknetEvent, Encoding = EventEncoding>
        + HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + HasBlobType<Blob = Vec<Felt>>
        + CanDeployContract
        + CanDeclareContract
        + CanSendSingleMessage
        + HasChainContractFields
        + HasAsyncErrorType,
    Chain::ContractClassHash: Eq + Debug + Hash,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanEncode<ViaCairo, Chain::Address>
        + CanEncode<ViaCairo, Chain::ContractClassHash>
        + CanEncode<
            ViaCairo,
            Product![
                Chain::Address,
                Chain::ContractClassHash,
                Chain::ContractClassHash,
                Chain::ContractClassHash
            ],
        > + CanEncode<ViaCairo, MsgRegisterClient>
        + CanEncode<ViaCairo, MsgRegisterApp>,
    EventEncoding: Async + HasEventContractFields<Chain>,
{
    async fn deploy_ibc_contracts(
        bootstrap: &Bootstrap,
        chain: &Chain,
    ) -> Result<(), Bootstrap::Error> {
        let cairo_encoding = <Chain as HasEncoding<AsFelt>>::encoding(chain);
        let event_encoding = <Chain as HasEncoding<AsStarknetEvent>>::encoding(chain);

        let erc20_class_hash = chain
            .declare_contract(bootstrap.erc20_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        bootstrap
            .log(
                &format!("declared ERC20 class: {erc20_class_hash:?}"),
                &LevelInfo,
            )
            .await;

        let ics20_class_hash = chain
            .declare_contract(bootstrap.ics20_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        bootstrap
            .log(
                &format!("declared ICS20 class: {ics20_class_hash:?}"),
                &LevelInfo,
            )
            .await;

        let comet_lib_class_hash = chain
            .declare_contract(bootstrap.comet_lib_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        bootstrap
            .log(
                &format!("declared Comet library class: {comet_lib_class_hash:?}"),
                &LevelInfo,
            )
            .await;

        let ics23_lib_class_hash = chain
            .declare_contract(bootstrap.ics23_lib_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        bootstrap
            .log(
                &format!("declared ICS23 library class: {ics23_lib_class_hash:?}"),
                &LevelInfo,
            )
            .await;

        let protobuf_lib_class_hash = chain
            .declare_contract(bootstrap.protobuf_lib_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        bootstrap
            .log(
                &format!("declared Protobuf library class: {protobuf_lib_class_hash:?}"),
                &LevelInfo,
            )
            .await;

        let comet_client_class_hash = chain
            .declare_contract(bootstrap.comet_client_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        bootstrap
            .log(
                &format!("declared Comet IBC client class: {comet_client_class_hash:?}"),
                &LevelInfo,
            )
            .await;

        let ibc_core_class_hash = chain
            .declare_contract(bootstrap.ibc_core_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        bootstrap
            .log(
                &format!("declared IBC core class: {ibc_core_class_hash:?}"),
                &LevelInfo,
            )
            .await;

        let ibc_core_address = {
            let call_data = cairo_encoding
                .encode(&protobuf_lib_class_hash)
                .map_err(Bootstrap::raise_error)?;

            chain
                .deploy_contract(&ibc_core_class_hash, false, &call_data)
                .await
                .map_err(Bootstrap::raise_error)?
        };

        bootstrap
            .log(
                &format!("deployed IBC core contract to address: {ibc_core_address:?}"),
                &LevelInfo,
            )
            .await;

        let comet_client_address = {
            let call_data = cairo_encoding
                .encode(&product![
                    ibc_core_address,
                    comet_lib_class_hash,
                    ics23_lib_class_hash,
                    protobuf_lib_class_hash,
                ])
                .map_err(Bootstrap::raise_error)?;

            chain
                .deploy_contract(&comet_client_class_hash, false, &call_data)
                .await
                .map_err(Bootstrap::raise_error)?
        };

        bootstrap
            .log(
                &format!("deployed Comet IBC client contract to address: {comet_client_address:?}"),
                &LevelInfo,
            )
            .await;

        let ics20_contract_address = {
            let owner_call_data = cairo_encoding
                .encode(&ibc_core_address)
                .map_err(Bootstrap::raise_error)?;

            let erc20_call_data = cairo_encoding
                .encode(&erc20_class_hash)
                .map_err(Bootstrap::raise_error)?;

            chain
                .deploy_contract(
                    &ics20_class_hash,
                    false,
                    &[owner_call_data, erc20_call_data].concat(),
                )
                .await
                .map_err(Bootstrap::raise_error)?
        };

        bootstrap
            .log(
                &format!("deployed ICS20 contract to address: {ics20_contract_address:?}"),
                &LevelInfo,
            )
            .await;

        {
            let register_client = MsgRegisterClient {
                client_type: short_string!("07-tendermint"),
                contract_address: comet_client_address,
            };

            let calldata = cairo_encoding
                .encode(&register_client)
                .map_err(Bootstrap::raise_error)?;

            let message =
                StarknetMessage::new(*ibc_core_address, selector!("register_client"), calldata);

            let response = chain
                .send_message(message)
                .await
                .map_err(Bootstrap::raise_error)?;

            bootstrap
                .log("registered comet client contract with ibc core", &LevelInfo)
                .await;
        }

        {
            // register the ICS20 contract with the IBC core contract

            let register_app = MsgRegisterApp {
                port_id: PortId::transfer(),
                contract_address: ics20_contract_address,
            };

            let register_call_data = cairo_encoding
                .encode(&register_app)
                .map_err(Bootstrap::raise_error)?;

            let message = StarknetMessage::new(
                *ibc_core_address,
                selector!("bind_port_id"),
                register_call_data,
            );

            let response = chain
                .send_message(message)
                .await
                .map_err(Bootstrap::raise_error)?;

            bootstrap
                .log("registered ICS20 contract with ibc core", &LevelInfo)
                .await;
        }

        chain
            .ibc_core_contract_address()
            .set(ibc_core_address)
            .map_err(|_| {
                Bootstrap::raise_error("failed to set ibc_core_contract_address on chain")
            })?;

        chain
            .ibc_client_contract_address()
            .set(comet_client_address)
            .map_err(|_| Bootstrap::raise_error("failed to set comet_client_address on chain"))?;

        chain
            .ibc_ics20_contract_address()
            .set(ics20_contract_address)
            .map_err(|_| Bootstrap::raise_error("failed to set ics20_contract_address on chain"))?;

        event_encoding
            .erc20_hashes()
            .set([erc20_class_hash].into())
            .map_err(|_| {
                Bootstrap::raise_error("failed to set erc20_class_hash on event encoding")
            })?;

        event_encoding
            .ics20_hashes()
            .set([ics20_class_hash].into())
            .map_err(|_| {
                Bootstrap::raise_error("failed to set ics20_class_hash on event encoding")
            })?;

        event_encoding
            .ibc_client_hashes()
            .set([comet_client_class_hash].into())
            .map_err(|_| {
                Bootstrap::raise_error("failed to set comet_client_class_hash on event encoding")
            })?;

        event_encoding
            .ibc_core_contract_addresses()
            .set([ibc_core_address].into())
            .map_err(|_| {
                Bootstrap::raise_error("failed to set ibc_core_address on event encoding")
            })?;

        Ok(())
    }
}

#[cgp_new_provider(ChainDriverBuilderComponent)]
impl<Bootstrap, InBuilder, Chain> ChainDriverBuilder<Bootstrap>
    for BuildChainAndDeployIbcContracts<InBuilder>
where
    Bootstrap: HasRuntimeType<Runtime: HasChildProcessType>
        + HasChainDriverType
        + HasChainType<Chain = Chain>
        + HasChainGenesisConfigType
        + HasChainNodeConfigType
        + CanDeployIbcContracts
        + HasAsyncErrorType,
    Bootstrap::ChainDriver: HasChain<Chain = Chain>,
    Chain: HasWalletType,
    InBuilder: ChainDriverBuilder<Bootstrap>,
{
    async fn build_chain_driver(
        bootstrap: &Bootstrap,
        genesis_config: Bootstrap::ChainGenesisConfig,
        chain_node_config: Bootstrap::ChainNodeConfig,
        wallets: BTreeMap<String, Chain::Wallet>,
        chain_process: Vec<ChildProcessOf<Bootstrap::Runtime>>,
    ) -> Result<Bootstrap::ChainDriver, Bootstrap::Error> {
        let chain_driver = InBuilder::build_chain_driver(
            bootstrap,
            genesis_config,
            chain_node_config,
            wallets,
            chain_process,
        )
        .await?;

        let chain = chain_driver.chain();

        bootstrap.deploy_ibc_contracts(chain).await?;

        Ok(chain_driver)
    }
}
