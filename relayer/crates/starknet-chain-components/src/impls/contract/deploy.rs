use cgp::core::error::CanRaiseError;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use starknet::contract::ContractFactory;
use starknet::core::types::{Felt, RevertedInvocation};
use starknet::macros::felt;
use starknet::signers::SigningKey;

use crate::traits::account::{CanRaiseAccountErrors, HasStarknetAccount};
use crate::traits::contract::deploy::ContractDeployer;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::contract_class::HasContractClassHashType;
use crate::types::tx_response::TxResponse;

pub struct DeployStarknetContract;

const DEFAULT_UDC_ADDRESS: Felt =
    felt!("0x041a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf");

impl<Chain> ContractDeployer<Chain> for DeployStarknetContract
where
    Chain: HasContractClassHashType<ContractClassHash = Felt>
        + HasAddressType<Address = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + CanPollTxResponse<TxHash = Felt, TxResponse = TxResponse>
        + HasStarknetAccount
        + CanRaiseAccountErrors
        + CanRaiseError<RevertedInvocation>,
{
    async fn deploy_contract(
        chain: &Chain,
        class_hash: &Felt,
        unique: bool,
        constructor_call_data: &Vec<Felt>,
    ) -> Result<Felt, Chain::Error> {
        let account = chain.account();

        let factory = ContractFactory::new_with_udc(*class_hash, account, DEFAULT_UDC_ADDRESS);

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

        Ok(deployed_address)
    }
}
