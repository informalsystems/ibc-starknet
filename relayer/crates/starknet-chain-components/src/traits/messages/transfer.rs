use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;

#[derive_component(TransferTokenMessageBuilderComponent, TransferTokenMessageBuilder<Chain>)]
pub trait CanBuildTransferTokenMessage:
    HasAddressType + HasAmountType + HasMessageType + HasErrorType
{
    fn build_transfer_token_message(
        &self,
        recipient: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<Self::Message, Self::Error>;
}
