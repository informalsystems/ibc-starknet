#[derive(Debug, clap::Parser, HasField)]
pub struct CreateClientArgs {
    /// Identifier of the chain that hosts the client
    #[clap(
        long = "target-chain",
        required = true,
        value_name = "TARGET_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    pub target_chain_id: String,

    /// Identifier of the chain targeted by the client
    #[clap(
        long = "counterparty-chain",
        required = true,
        value_name = "COUNTERPARTY_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    pub counterparty_chain_id: String,
}
