use cgp_core::prelude::*;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;

#[derive_component(TokenTransferComponent, TokenTransferer<Chain>)]
#[async_trait]
pub trait CanTransferToken: HasAddressType + HasAmountType + HasErrorType {
    async fn transfer_token(
        &self,
        recipient: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
