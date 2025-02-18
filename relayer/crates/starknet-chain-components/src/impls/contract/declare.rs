use std::sync::Arc;

use cairo_lang_starknet_classes::casm_contract_class::{
    CasmContractClass, StarknetSierraCompilationError,
};
use cairo_lang_starknet_classes::contract_class::ContractClass;
use cgp::core::error::CanRaiseAsyncError;
use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use starknet::accounts::Account;
use starknet::core::types::contract::{
    CompiledClass, ComputeClassHashError, JsonError, SierraClass,
};
use starknet::core::types::{BlockId, BlockTag, Felt, RevertedInvocation};
use starknet::providers::Provider;

use crate::components::chain::ContractDeclarerComponent;
use crate::traits::account::{CanRaiseAccountErrors, HasStarknetAccount};
use crate::traits::contract::declare::ContractDeclarer;
use crate::traits::provider::HasStarknetProvider;
use crate::traits::types::contract_class::{HasContractClassHashType, HasContractClassType};
use crate::types::tx_response::TxResponse;

pub struct DeclareSierraContract;

#[cgp_provider(ContractDeclarerComponent)]
impl<Chain> ContractDeclarer<Chain> for DeclareSierraContract
where
    Chain: HasContractClassType<ContractClass = SierraClass>
        + HasContractClassHashType<ContractClassHash = Felt>
        + HasStarknetProvider
        + HasStarknetAccount
        + CanPollTxResponse<TxHash = Felt, TxResponse = TxResponse>
        + CanRaiseAccountErrors
        + CanRaiseAsyncError<serde_json::error::Error>
        + CanRaiseAsyncError<JsonError>
        + CanRaiseAsyncError<ComputeClassHashError>
        + CanRaiseAsyncError<RevertedInvocation>
        + CanRaiseAsyncError<StarknetSierraCompilationError>,
{
    async fn declare_contract(
        chain: &Chain,
        contract_class: &SierraClass,
    ) -> Result<Felt, Chain::Error> {
        let provider = chain.provider();
        let account = chain.account();

        let class_hash = contract_class.class_hash().map_err(Chain::raise_error)?;

        let class_exist_result = provider
            .get_class(BlockId::Tag(BlockTag::Pending), class_hash)
            .await;

        if class_exist_result.is_ok() {
            return Ok(class_hash);
        }

        // Compile Sierra class to Casm, following code in starkli
        let casm_class_hash = {
            let mut class = contract_class.clone();
            class.abi.clear();

            let sierra_class_json = serde_json::to_string(&class).map_err(Chain::raise_error)?;

            let contract_class: ContractClass =
                serde_json::from_str(&sierra_class_json).map_err(Chain::raise_error)?;

            let casm_contract =
                CasmContractClass::from_contract_class(contract_class, false, 180000)
                    .map_err(Chain::raise_error)?;

            let casm_class_json =
                serde_json::to_string(&casm_contract).map_err(Chain::raise_error)?;

            let casm_class = serde_json::from_str::<CompiledClass>(&casm_class_json)
                .map_err(Chain::raise_error)?;

            casm_class.class_hash().map_err(Chain::raise_error)?
        };

        let flattened_class = contract_class
            .clone()
            .flatten()
            .map_err(Chain::raise_error)?;

        let declaration = account.declare_v3(Arc::new(flattened_class), casm_class_hash);

        let declare_result = declaration.send().await.map_err(Chain::raise_error)?;

        let tx_response = chain
            .poll_tx_response(&declare_result.transaction_hash)
            .await?;

        if let Some(reverted) = tx_response.is_reverted() {
            return Err(Chain::raise_error(reverted));
        }

        Ok(declare_result.class_hash)
    }
}
