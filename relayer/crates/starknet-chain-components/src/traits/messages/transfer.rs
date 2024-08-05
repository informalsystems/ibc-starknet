use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

use crate::traits::types::address::HasAddressType;
use crate::traits::types::amount::HasAmountType;

#[derive_component(TransferTokenMessageBuilderComponent, TransferTokenMessageBuilder<Chain>)]
pub trait CanBuildTransferTokenMessage: HasAddressType + HasAmountType + HasMessageType {
    fn build_transfer_token_message(
        &self,
        token_address: &Self::Address,
        recipient: &Self::Address,
        amount: &Self::Amount,
    ) -> Self::Message;
}
