use cgp::prelude::HasAsyncErrorType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::HasIbcChainTypes;
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::timestamp::HasTimeoutType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilder;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain::traits::types::memo::HasMemoType;

pub struct BuildStarknetIbcTransferMessage;

impl<Chain, Counterparty> IbcTokenTransferMessageBuilder<Chain, Counterparty>
    for BuildStarknetIbcTransferMessage
where
    Chain: HasAsyncErrorType
        + HasAmountType
        + HasMemoType
        + HasMessageType
        + HasHeightType
        + HasTimeoutType
        + HasIbcChainTypes<Counterparty>,
    Counterparty: HasAddressType,
{
    async fn build_ibc_token_transfer_message(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        recipient_address: &Counterparty::Address,
        amount: &Chain::Amount,
        memo: &Chain::Memo,
        timeout_height: Option<&Chain::Height>,
        timeout_time: Option<&Chain::Timeout>,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}
