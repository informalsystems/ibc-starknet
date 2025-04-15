use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_test_components::chain::traits::types::address::HasAddressType;

#[cgp_component {
  name: TokenTransferComponent,
  provider: TokenTransferer,
  context: Chain,
}]
#[async_trait]
pub trait CanTransferToken: HasAddressType + HasAmountType + HasAsyncErrorType {
    async fn transfer_token(
        &self,
        recipient: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
