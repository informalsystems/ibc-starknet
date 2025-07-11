use std::sync::Arc;

use cairo_lang_starknet_classes::casm_contract_class::{
    CasmContractClass, StarknetSierraCompilationError,
};
use cairo_lang_starknet_classes::contract_class::ContractClass;
use cgp::core::error::CanRaiseAsyncError;
use hermes_core::relayer_components::transaction::traits::{CanPollTxResponse, HasDefaultSigner};
use hermes_prelude::*;
use starknet::accounts::Account;
use starknet::core::types::contract::{
    CompiledClass, ComputeClassHashError, JsonError, SierraClass,
};
use starknet::core::types::{BlockId, BlockTag, Felt, RevertedInvocation};
use starknet::providers::Provider;

use crate::traits::{
    CanBuildAccountFromSigner, CanUseStarknetAccount, ContractDeclarer, ContractDeclarerComponent,
    HasContractClassHashType, HasContractClassType, HasStarknetClient,
};
use crate::types::TxResponse;

#[cgp_new_provider(ContractDeclarerComponent)]
impl<Chain> ContractDeclarer<Chain> for DeclareSierraContract
where
    Chain: HasContractClassType<ContractClass = SierraClass>
        + HasContractClassHashType<ContractClassHash = Felt>
        + HasStarknetClient<Client: Provider>
        + HasDefaultSigner
        + CanBuildAccountFromSigner
        + CanPollTxResponse<TxHash = Felt, TxResponse = TxResponse>
        + CanUseStarknetAccount
        + CanRaiseAsyncError<serde_json::error::Error>
        + CanRaiseAsyncError<JsonError>
        + CanRaiseAsyncError<ComputeClassHashError>
        + CanRaiseAsyncError<RevertedInvocation>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<StarknetSierraCompilationError>,
{
    async fn declare_contract(
        chain: &Chain,
        contract_class: &SierraClass,
    ) -> Result<Felt, Chain::Error> {
        let provider = chain.provider();
        let signer = chain.get_default_signer();
        let account = chain.build_account_from_signer(signer);

        let class_hash = contract_class.class_hash().map_err(Chain::raise_error)?;

        let class_exist_result = provider
            .get_class(BlockId::Tag(BlockTag::Latest), class_hash)
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

            // https://docs.starknet.io/resources/chain-info
            // max contract bytecode size: 81_920
            let casm_contract =
                CasmContractClass::from_contract_class(contract_class, false, 81_920)
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

        let fee_estimation = declaration
            .estimate_fee()
            .await
            .map_err(Chain::raise_error)?;

        // While using Madara this code is commented out due to the configured max gas being 0.
        // This causes: Error: StarknetError: ValidationFailure("Max L1Gas price (0) is lower than the actual gas price: 1.")
        // This is blocked by Madara's starknet version update
        // See: https://www.starknet.io/developers/roadmap/
        /*
        // starknet v3 transactions requires all fee bound present.
        let l1_gas = core::cmp::max(
            1,
            fee_estimation
                .gas_consumed
                .try_into()
                .map_err(|_| Chain::raise_error("failed to convert felt to u64"))?,
        );
        let l1_data_gas = core::cmp::max(
            1,
            fee_estimation
                .data_gas_consumed
                .try_into()
                .map_err(|_| Chain::raise_error("failed to convert felt to u64"))?,
        );
        let l2_gas = core::cmp::max(
            1,
            fee_estimation
                .gas_consumed
                .try_into()
                .map_err(|_| Chain::raise_error("failed to convert felt to u64"))?,
        );*/

        let declare_result = declaration
            //.gas(l1_gas)
            .send()
            .await
            .map_err(Chain::raise_error)?;

        let tx_response = chain
            .poll_tx_response(&declare_result.transaction_hash)
            .await?;

        if let Some(reverted) = tx_response.is_reverted() {
            return Err(Chain::raise_error(reverted));
        }

        Ok(declare_result.class_hash)
    }
}
