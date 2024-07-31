use cgp_core::prelude::*;

use crate::traits::types::address::HasAddressType;
use crate::traits::types::amount::HasAmountType;

#[derive_component(TokenTransferComponent, TokenTransferer<Chain>)]
#[async_trait]
pub trait CanTransferToken: HasAddressType + HasAmountType + HasErrorType {
    async fn transfer_token(
        &self,
        token_address: &Self::Address,
        recipient: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
