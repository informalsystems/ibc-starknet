use hermes_prelude::*;

#[derive(Debug, clap::Parser, HasField)]
pub struct StartRelayerArgs {
    /// Identifier of Starknet chain
    #[clap(
        long = "starknet-chain-id",
        required = true,
        value_name = "STARKNET_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    chain_id_a: String,

    /// Identifier of Starknet client
    #[clap(
        long = "starknet-client-id",
        required = true,
        value_name = "STARKNET_CLIENT_ID",
        help_heading = "REQUIRED"
    )]
    client_id_a: String,

    /// Identifier of Cosmos chain
    #[clap(
        long = "cosmos-chain-id",
        required = true,
        value_name = "COSMOS_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    chain_id_b: String,

    /// Identifier of Cosmos client
    #[clap(
        long = "cosmos-client-id",
        required = true,
        value_name = "COSMOS_CLIENT_ID",
        help_heading = "REQUIRED"
    )]
    client_id_b: String,

    #[clap(long = "clear-past-blocks", required = false)]
    clear_past_blocks: Option<humantime::Duration>,

    #[clap(long = "stop-after-blocks", required = false)]
    stop_after_blocks: Option<humantime::Duration>,

    #[clap(long = "refresh-rate-a-to-b", required = false)]
    refresh_rate_a: Option<humantime::Duration>,

    #[clap(long = "refresh-rate-b-to-a", required = false)]
    refresh_rate_b: Option<humantime::Duration>,
}
