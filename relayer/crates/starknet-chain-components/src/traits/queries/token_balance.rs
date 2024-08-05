use cgp_core::prelude::*;

use crate::traits::types::address::HasAddressType;
use crate::traits::types::amount::HasAmountType;

#[derive_component(TokenBalanceQuerierComponent, TokenBalanceQuerier<Chain>)]
#[async_trait]
pub trait CanQueryTokenBalance: HasAddressType + HasAmountType + HasErrorType {
    async fn query_token_balance(
        &self,
        token_address: &Self::Address,
        account_address: &Self::Address,
    ) -> Result<Self::Amount, Self::Error>;
}