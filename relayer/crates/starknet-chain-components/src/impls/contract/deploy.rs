use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_core::relayer_components::transaction::traits::{CanPollTxResponse, HasDefaultSigner};
use hermes_prelude::*;
use starknet::contract::ContractFactory;
use starknet::core::types::{Felt, RevertedInvocation};
use starknet::macros::felt;
use starknet::signers::SigningKey;

use crate::impls::StarknetAddress;
use crate::traits::{
    CanBuildAccountFromSigner, CanUseStarknetAccount, ContractDeployer, ContractDeployerComponent,
    HasBlobType, HasContractClassHashType,
};
use crate::types::TxResponse;

pub struct DeployStarknetContract;

const DEFAULT_UDC_ADDRESS: Felt =
    felt!("0x041a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf");

#[cgp_provider(ContractDeployerComponent)]
impl<Chain> ContractDeployer<Chain> for DeployStarknetContract
where
    Chain: HasContractClassHashType<ContractClassHash = Felt>
        + HasAddressType<Address = StarknetAddress>
        + HasBlobType<Blob = Vec<Felt>>
        + CanPollTxResponse<TxHash = Felt, TxResponse = TxResponse>
        + HasDefaultSigner
        + CanBuildAccountFromSigner
        + CanUseStarknetAccount
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<RevertedInvocation>,
{
    async fn deploy_contract(
        chain: &Chain,
        class_hash: &Felt,
        unique: bool,
        constructor_call_data: &Vec<Felt>,
    ) -> Result<StarknetAddress, Chain::Error> {
        let signer = chain.get_default_signer();
        let account = chain.build_account_from_signer(signer);

        let factory = ContractFactory::new_with_udc(*class_hash, account, DEFAULT_UDC_ADDRESS);

        let salt = SigningKey::from_random().secret_scalar();

        let contract_deployment = factory.deploy_v3(constructor_call_data.clone(), salt, unique);

        let deployed_address = contract_deployment.deployed_address();

        let fee_estimation = contract_deployment
            .estimate_fee()
            .await
            .map_err(Chain::raise_error)?;

        // starknet v3 transactions requires all fee bound present.
        let l1_gas = core::cmp::max(
            1,
            fee_estimation
                .l1_gas_consumed
                .try_into()
                .map_err(|_| Chain::raise_error("failed to convert felt to u64"))?,
        );
        let l1_data_gas = core::cmp::max(
            1,
            fee_estimation
                .l1_data_gas_consumed
                .try_into()
                .map_err(|_| Chain::raise_error("failed to convert felt to u64"))?,
        );
        let l2_gas = core::cmp::max(
            1,
            fee_estimation
                .l2_gas_consumed
                .try_into()
                .map_err(|_| Chain::raise_error("failed to convert felt to u64"))?,
        );

        let tx_hash = contract_deployment
            .l1_gas(l1_gas)
            .l1_data_gas(l1_data_gas)
            .l2_gas(l2_gas)
            .send()
            .await
            .map_err(Chain::raise_error)?
            .transaction_hash;

        let tx_response = chain.poll_tx_response(&tx_hash).await?;

        if let Some(reverted) = tx_response.is_reverted() {
            return Err(Chain::raise_error(reverted));
        }

        Ok(deployed_address.into())
    }
}
