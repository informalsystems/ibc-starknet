use hermes_core::chain_components::traits::CanSendSingleMessage;
use hermes_prelude::*;

use crate::traits::{CanBuildInvokeContractMessage, ContractInvoker, ContractInvokerComponent};

pub struct InvokeStarknetContract;

#[cgp_provider(ContractInvokerComponent)]
impl<Chain> ContractInvoker<Chain> for InvokeStarknetContract
where
    Chain: CanBuildInvokeContractMessage + CanSendSingleMessage,
{
    async fn invoke_contract(
        chain: &Chain,
        contract_address: &Chain::Address,
        entry_point_selector: &Chain::Selector,
        calldata: &Chain::Blob,
    ) -> Result<Chain::MessageResponse, Chain::Error> {
        let message =
            chain.build_invoke_contract_message(contract_address, entry_point_selector, calldata);

        chain.send_message(message).await
    }
}
