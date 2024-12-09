use cgp::prelude::*;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;

#[cgp_component {
  name: TokenTransferComponent,
  provider: TokenTransferer,
  context: Chain,
}]
#[async_trait]
pub trait CanTransferToken: HasAddressType + HasAmountType + HasErrorType {
    async fn transfer_token(
        &self,
        recipient: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
