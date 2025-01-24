use cgp::prelude::HasAsyncErrorType;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::timestamp::HasTimeoutType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilder;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain::traits::types::memo::HasMemoType;
use ibc::core::host::types::identifiers::PortId;
use ibc::primitives::Timestamp;

use crate::impls::types::message::StarknetMessage;
use crate::types::amount::StarknetAmount;
use crate::types::channel_id::ChannelId;
use crate::types::messages::ibc::ibc_transfer::{MsgTransfer, TransferPacketData};

pub struct BuildStarknetIbcTransferMessage;

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
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
        recipient_address: &Counterparty::Address,
        amount: &StarknetAmount,
        memo: &Option<String>,
        timeout_height: Option<&u64>,
        timeout_time: Option<&Timestamp>,
    ) -> Result<Chain::Message, Chain::Error> {
        // let packet_data =
        //     TransferPacketData {
        //         denom,
        //         amount,
        //         sender,
        //         receiver,
        //         memo,
        //     };

        // let message =
        //     MsgTransfer {
        //         port_id_on_a: port_id.clone(),
        //         chan_id_on_a: channel_id.clone(),
        //         packet_data: starknet_ic20_packet_data,
        //         timeout_height_on_b: Height {
        //             revision_number: 0,
        //             revision_height: 0,
        //         },
        //         timeout_timestamp_on_b: Timestamp {
        //             timestamp: u64::try_from(current_starknet_time.unix_timestamp()).unwrap()
        //                 + 1800,
        //         },
        //     };

        todo!()
    }
}
