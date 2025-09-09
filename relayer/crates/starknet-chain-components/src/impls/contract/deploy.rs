use hermes_core::chain_type_components::traits::HasAddressType;
use hermes_core::relayer_components::transaction::traits::{CanPollTxResponse, HasDefaultSigner};
use hermes_prelude::*;
use starknet::contract::{ContractFactory, UdcSelector};
use starknet::core::types::{Felt, RevertedInvocation};
use starknet::signers::SigningKey;

use crate::impls::StarknetAddress;
use crate::traits::{
    CanBuildAccountFromSigner, CanUseStarknetAccount, ContractDeployer, ContractDeployerComponent,
    HasBlobType, HasContractClassHashType,
};
use crate::types::TxResponse;

pub struct DeployStarknetContract;

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

        let factory = ContractFactory::new_with_udc(*class_hash, account, UdcSelector::Legacy);

        let salt = SigningKey::from_random().secret_scalar();

        let contract_deployment = factory.deploy_v3(constructor_call_data.clone(), salt, unique);

        let deployed_address = contract_deployment.deployed_address();

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
