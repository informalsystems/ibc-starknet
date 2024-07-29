use cgp_core::prelude::*;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;

use crate::traits::types::address::HasAddressType;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasMethodSelectorType;

#[derive_component(ContractInvokerComponent, ContractInvoker<Chain>)]
#[async_trait]
pub trait CanInvokeContract:
    HasAddressType + HasMethodSelectorType + HasBlobType + HasTransactionHashType + HasErrorType
{
    async fn invoke_contract(
        &self,
        contract_address: &Self::Address,
        selector: &Self::MethodSelector,
        call_data: &Self::Blob,
    ) -> Result<Self::TxHash, Self::Error>;
}
