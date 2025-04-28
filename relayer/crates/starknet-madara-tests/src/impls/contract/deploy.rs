use cgp::prelude::*;
use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_core::relayer_components::transaction::traits::{CanPollTxResponse, HasDefaultSigner};
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::traits::account::CanBuildAccountFromSigner;
use hermes_starknet_chain_components::traits::contract::deploy::{
    ContractDeployer, ContractDeployerComponent,
};
use hermes_starknet_chain_components::traits::types::blob::HasBlobType;
use hermes_starknet_chain_components::traits::types::contract_class::HasContractClassHashType;
use starknet_v13::contract::ContractFactory;
use starknet_v13::core::types::{Felt, RevertedInvocation};
use starknet_v13::signers::SigningKey;

use crate::traits::CanUseStarknetAccount;
use crate::types::TxResponse;

pub struct DeployStarknetContract;

const DEFAULT_UDC_ADDRESS: Felt = Felt::from_raw([
    121672436446604875,
    9333317513348225193,
    15685625669053253235,
    15144800532519055890,
]);

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

        let tx_hash = contract_deployment
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
