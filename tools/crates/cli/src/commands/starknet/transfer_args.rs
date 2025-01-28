use std::str::FromStr;

use cgp::prelude::*;
use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_components::traits::command::CommandRunner;
use hermes_cli_components::traits::output::HasOutputType;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_starknet_chain_components::types::channel_id::ChannelId;
use hermes_starknet_chain_components::types::cosmos::height::Height;
use hermes_starknet_chain_components::types::cosmos::timestamp::Timestamp;
use hermes_starknet_chain_components::types::messages::ibc::channel::PortId;
use hermes_starknet_chain_components::types::messages::ibc::denom::{
    Denom, PrefixedDenom, TracePrefix,
};
use hermes_starknet_chain_components::types::messages::ibc::ibc_transfer::{
    MsgTransfer, Participant, TransferPacketData,
};
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use ibc::core::host::types::identifiers::PortId as IbcPortId;
use starknet::core::types::Felt;

use crate::contexts::app::ToolApp;

#[derive(Debug, clap::Parser, HasField)]
pub struct TransferArgs {
    /// Amount to transfer
    #[clap(
        long = "amount",
        required = true,
        value_name = "AMOUNT",
        help_heading = "REQUIRED"
    )]
    pub amount: String,

    /// Denom of the amount to transfer
    #[clap(
        long = "denom",
        required = true,
        value_name = "DENOM",
        help_heading = "REQUIRED"
    )]
    pub denom: String,

    /// Address of the receiver
    #[clap(
        long = "receiver",
        required = true,
        value_name = "RECEIVER",
        help_heading = "REQUIRED"
    )]
    pub receiver: String,

    /// Address of the sender
    #[clap(
        long = "sender",
        required = true,
        value_name = "SENDER",
        help_heading = "REQUIRED"
    )]
    pub sender: String,

    /// ID of the channel used for the transfer
    #[clap(
        long = "channel-id",
        required = true,
        value_name = "CHANNEL_ID",
        help_heading = "REQUIRED"
    )]
    pub channel_id: String,
}

pub struct RunTransferArgs;

#[async_trait]
impl CommandRunner<ToolApp, TransferArgs> for RunTransferArgs {
    async fn run_command(
        app: &ToolApp,
        args: &TransferArgs,
    ) -> Result<<ToolApp as HasOutputType>::Output, <ToolApp as HasErrorType>::Error> {
        let builder = app.load_builder().await?;
        let logger = app.logger();

        let starknet_chain = builder.build_chain().await?;

        let ics20_port = IbcPortId::transfer();

        let denom = PrefixedDenom {
            trace_path: vec![TracePrefix {
                port_id: ics20_port.to_string(),
                channel_id: args.channel_id.clone(),
            }],
            base: Denom::Hosted(args.denom.clone()),
        };

        let amount_u128: u128 = args.amount.parse()?;

        let sender = Participant::Native(Felt::from_str(&args.sender)?);
        let receiver = Participant::External(args.receiver.clone());

        let current_starknet_time = starknet_chain.query_chain_status().await?.time;

        let starknet_ic20_packet_data = TransferPacketData {
            denom,
            amount: amount_u128.into(),
            sender,
            receiver,
            memo: "demo transfer".to_owned(),
        };

        let msg_transfer = MsgTransfer {
            port_id_on_a: PortId {
                port_id: ics20_port.to_string(),
            },
            chan_id_on_a: ChannelId {
                channel_id: args.channel_id.clone(),
            },
            packet_data: starknet_ic20_packet_data,
            timeout_height_on_b: Height {
                revision_number: 0,
                revision_height: 0,
            },
            timeout_timestamp_on_b: Timestamp {
                timestamp: u64::try_from(current_starknet_time.unix_timestamp())? + 1800,
            },
        };

        let cairo_encoding = StarknetCairoEncoding;

        let call_data = cairo_encoding.encode(&msg_transfer)?;

        let call_data_str = call_data
            .iter()
            .map(|data| data.to_hex_string())
            .collect::<Vec<String>>()
            .join(" ");

        logger
            .log(
                &format!(
                    "Arguments to send transaction using `starkli invoke` are: {}",
                    call_data_str
                ),
                &LevelInfo,
            )
            .await;

        Ok(())
    }
}
