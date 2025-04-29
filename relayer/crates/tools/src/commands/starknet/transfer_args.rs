use hermes_cli_components::traits::{CommandRunner, CommandRunnerComponent, HasOutputType};
use hermes_core::encoding_components::traits::CanEncode;
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelInfo;
use hermes_prelude::*;
use hermes_starknet_chain_components::types::{
    Denom, Height, MsgTransfer, PortId, PrefixedDenom, Timestamp, TracePrefix,
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

    /// ID of the channel used for the transfer
    #[clap(
        long = "channel-id",
        required = true,
        value_name = "CHANNEL_ID",
        help_heading = "REQUIRED"
    )]
    pub channel_id: String,

    /// Timeout timestamp for the transfer
    #[clap(
        long = "timeout-timestamp",
        required = true,
        value_name = "TIMEOUT_TIMESTAMP",
        help_heading = "REQUIRED"
    )]
    pub timeout_timestamp: u64,
}

pub struct RunTransferArgs;

#[cgp_provider(CommandRunnerComponent)]
#[async_trait]
impl CommandRunner<ToolApp, TransferArgs> for RunTransferArgs {
    async fn run_command(
        app: &ToolApp,
        args: &TransferArgs,
    ) -> Result<<ToolApp as HasOutputType>::Output, <ToolApp as HasErrorType>::Error> {
        let ics20_port = IbcPortId::transfer();

        // If the passed denom starts with 0x this means it is an ERC20 token
        // Else it is a Cosmos token
        let denom = if args.denom.starts_with("0x") {
            PrefixedDenom {
                trace_path: vec![],
                base: Denom::Native(Felt::from_hex(&args.denom)?.into()),
            }
        } else {
            PrefixedDenom {
                trace_path: vec![TracePrefix {
                    port_id: ics20_port.to_string(),
                    channel_id: args.channel_id.clone(),
                }],
                base: Denom::Hosted(args.denom.clone()),
            }
        };

        let amount_u128: u128 = args.amount.parse()?;

        let msg_transfer = MsgTransfer {
            port_id_on_a: PortId::transfer(),
            chan_id_on_a: args.channel_id.parse()?,
            denom,
            amount: amount_u128.into(),
            receiver: args.receiver.clone(),
            memo: "demo transfer".to_owned(),
            timeout_height_on_b: Height {
                revision_number: 0,
                revision_height: 0,
            },
            timeout_timestamp_on_b: Timestamp::from_nanoseconds(
                args.timeout_timestamp * 1_000_000_000,
            ),
        };

        let cairo_encoding = StarknetCairoEncoding;

        let call_data = cairo_encoding.encode(&msg_transfer)?;

        let call_data_str = call_data
            .iter()
            .map(|data| data.to_hex_string())
            .collect::<Vec<String>>()
            .join(" ");

        app.log(
            &format!("Arguments to send transaction using `starkli invoke` are: {call_data_str}"),
            &LevelInfo,
        )
        .await;

        Ok(())
    }
}
