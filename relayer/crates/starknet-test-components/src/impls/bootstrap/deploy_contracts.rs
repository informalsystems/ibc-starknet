use std::collections::BTreeMap;
use std::sync::OnceLock;

use cgp::extra::runtime::HasRuntimeType;
use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::{
    ChainDriverBuilder, ChainDriverBuilderComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::impls::types::message::StarknetMessage;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::types::blob::HasBlobType;
use hermes_starknet_chain_components::traits::types::contract_class::{
    ContractClassOf, HasContractClassType,
};
use hermes_starknet_chain_components::types::register::MsgRegisterClient;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::types::chain::{HasChain, HasChainType};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use starknet::accounts::Call;
use starknet::core::types::Felt;
use starknet::macros::{selector, short_string};

use crate::traits::{CanDeployIbcContracts, IbcContractsDeployer, IbcContractsDeployerComponent};

#[cgp_auto_getter]
pub trait HasIbcContracts: HasChainType<Chain: HasContractClassType> {
    fn erc20_contract(&self) -> &ContractClassOf<Self::Chain>;

    fn ics20_contract(&self) -> &ContractClassOf<Self::Chain>;

    fn ibc_core_contract(&self) -> &ContractClassOf<Self::Chain>;

    fn comet_client_contract(&self) -> &ContractClassOf<Self::Chain>;
}

#[cgp_auto_getter]
pub trait HasChainContractFields: HasAddressType {
    fn ibc_core_contract_address(&self) -> &OnceLock<Self::Address>;

    fn ibc_client_contract_address(&self) -> &OnceLock<Self::Address>;
}

#[cgp_new_provider(IbcContractsDeployerComponent)]
impl<Bootstrap, Chain, CairoEncoding, EventEncoding> IbcContractsDeployer<Bootstrap>
    for DeployIbcContract
where
    Bootstrap: HasLogger
        + HasChainType<Chain = Chain>
        + HasIbcContracts
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
    Bootstrap::Logger: CanLog<LevelInfo>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanEncode<ViaCairo, Chain::Address>
        + CanEncode<ViaCairo, MsgRegisterClient>,
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

        let ics20_class_hash = chain
            .declare_contract(bootstrap.ics20_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        let ibc_core_class_hash = chain
            .declare_contract(bootstrap.ibc_core_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        let comet_client_class_hash = chain
            .declare_contract(bootstrap.comet_client_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        let ibc_core_address = chain
            .deploy_contract(&ibc_core_class_hash, false, &Vec::new())
            .await
            .map_err(Bootstrap::raise_error)?;

        let comet_client_address = {
            let call_data = cairo_encoding
                .encode(&ibc_core_address)
                .map_err(Bootstrap::raise_error)?;

            chain
                .deploy_contract(&comet_client_class_hash, false, &call_data)
                .await
                .map_err(Bootstrap::raise_error)?
        };

        {
            let register_client = MsgRegisterClient {
                client_type: short_string!("07-tendermint"),
                contract_address: comet_client_address,
            };

            let calldata = cairo_encoding
                .encode(&register_client)
                .map_err(Bootstrap::raise_error)?;

            let call = Call {
                to: *ibc_core_address,
                selector: selector!("register_client"),
                calldata,
            };

            let message = StarknetMessage::new(call);

            let response = chain
                .send_message(message)
                .await
                .map_err(Bootstrap::raise_error)?;
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
            .map_err(|_| {
                Bootstrap::raise_error("failed to set ibc_core_contract_address on chain")
            })?;

        Ok(())
    }
}

#[cgp_new_provider(ChainDriverBuilderComponent)]
impl<Bootstrap, InBuilder, Chain> ChainDriverBuilder<Bootstrap>
    for BuildChainAndDeployIbcContracts<InBuilder>
where
    Bootstrap: HasRuntimeType<Runtime: HasChildProcessType>
        + HasChainDriverType<Chain = Chain>
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
        chain_process: ChildProcessOf<Bootstrap::Runtime>,
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

        bootstrap.deploy_ibc_contracts(&chain).await?;

        Ok(chain_driver)
    }
}
