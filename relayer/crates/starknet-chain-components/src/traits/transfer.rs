use cgp::prelude::*;
use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType};

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
