use cgp::prelude::*;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::timestamp::HasTimeoutType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::messages::ibc_transfer::{
    IbcTokenTransferMessageBuilder, IbcTokenTransferMessageBuilderComponent,
};
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain::traits::types::memo::HasMemoType;
use ibc::core::host::types::identifiers::PortId;
use ibc::primitives::Timestamp;

use crate::impls::types::message::StarknetMessage;
use crate::types::amount::StarknetAmount;
use crate::types::channel_id::ChannelId;

pub struct BuildStarknetIbcTransferMessage;

#[cgp_provider(IbcTokenTransferMessageBuilderComponent)]
impl<Chain, Counterparty> IbcTokenTransferMessageBuilder<Chain, Counterparty>
    for BuildStarknetIbcTransferMessage
where
    Chain: HasAsyncErrorType
        + HasAmountType<Amount = StarknetAmount>
        + HasMemoType<Memo = Option<String>>
        + HasMessageType<Message = StarknetMessage>
        + HasHeightType<Height = u64>
        + HasTimeoutType<Timeout = Timestamp>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = PortId>,
    Counterparty: HasAddressType,
{
    async fn build_ibc_token_transfer_message(
        _chain: &Chain,
        _channel_id: &ChannelId,
        _port_id: &PortId,
        _recipient_address: &Counterparty::Address,
        _amount: &StarknetAmount,
        _memo: &Option<String>,
        _timeout_height: Option<&u64>,
        _timeout_time: Option<&Timestamp>,
    ) -> Result<Chain::Message, Chain::Error> {
        // FIXME: Implement the logic to build the token transfer message
        todo!()
    }
}
