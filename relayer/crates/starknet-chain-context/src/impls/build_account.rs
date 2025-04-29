use core::str::FromStr;
use std::sync::Arc;

use hermes_core::chain_components::traits::HasChainId;
use hermes_core::relayer_components::transaction::traits::HasSignerType;
use hermes_prelude::*;
use hermes_starknet_chain_components::traits::account::{
    AccountFromSignerBuilder, AccountFromSignerBuilderComponent, HasStarknetAccountType,
};
use hermes_starknet_chain_components::traits::client::HasStarknetClient;
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use ibc::core::host::types::identifiers::ChainId;
use starknet::accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet::core::types::Felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use starknet::signers::{LocalWallet, SigningKey};

use crate::types::StarknetAccount;

#[cgp_new_provider(AccountFromSignerBuilderComponent)]
impl<Chain> AccountFromSignerBuilder<Chain> for BuildStarknetAccount
where
    Chain: HasStarknetClient<Client = Arc<JsonRpcClient<HttpTransport>>>
        + HasChainId<ChainId = ChainId>
        + HasStarknetAccountType<Account = StarknetAccount>
        + HasSignerType<Signer = StarknetWallet>,
{
    fn build_account_from_signer(chain: &Chain, signer: &StarknetWallet) -> StarknetAccount {
        SingleOwnerAccount::new(
            chain.provider().clone(),
            LocalWallet::from_signing_key(SigningKey::from_secret_scalar(signer.signing_key)),
            *signer.account_address,
            Felt::from_str(chain.chain_id().as_str()).unwrap(),
            ExecutionEncoding::New,
        )
    }
}
