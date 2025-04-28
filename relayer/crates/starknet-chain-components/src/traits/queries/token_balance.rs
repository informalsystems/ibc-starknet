use cgp::prelude::*;
use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType};

#[cgp_component {
  name: TokenBalanceQuerierComponent,
  provider: TokenBalanceQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryTokenBalance: HasAddressType + HasAmountType + HasAsyncErrorType {
    async fn query_token_balance(
        &self,
        token_address: &Self::Address,
        account_address: &Self::Address,
    ) -> Result<Self::Amount, Self::Error>;
}
