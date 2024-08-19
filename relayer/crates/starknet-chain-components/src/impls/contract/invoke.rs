use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;

use crate::traits::contract::invoke::ContractInvoker;
use crate::traits::contract::message::CanBuildInvokeContractMessage;

pub struct InvokeStarknetContract;

impl<Chain> ContractInvoker<Chain> for InvokeStarknetContract
where
    Chain: CanBuildInvokeContractMessage + CanSendSingleMessage,
{
    async fn invoke_contract(
        chain: &Chain,
        contract_address: &Chain::Address,
        entry_point_selector: &Chain::Selector,
        calldata: &Chain::Blob,
    ) -> Result<Vec<Chain::Event>, Chain::Error> {
        let message =
            chain.build_invoke_contract_message(contract_address, entry_point_selector, calldata);

        chain.send_message(message).await
    }
}
