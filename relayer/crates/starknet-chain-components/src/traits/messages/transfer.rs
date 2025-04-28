use cgp::prelude::*;
use hermes_core::chain_components::traits::HasMessageType;
use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType};

#[cgp_component {
  name: TransferTokenMessageBuilderComponent,
  provider: TransferTokenMessageBuilder,
  context: Chain,
}]
pub trait CanBuildTransferTokenMessage:
    HasAddressType + HasAmountType + HasMessageType + HasAsyncErrorType
{
    fn build_transfer_token_message(
        &self,
        recipient: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<Self::Message, Self::Error>;
}
