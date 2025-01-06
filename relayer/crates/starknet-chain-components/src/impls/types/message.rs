use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::message::ProvideMessageType;
use ibc::core::client::types::Height as CosmosHeight;
use starknet::accounts::Call;

#[derive(Clone)]
pub struct StarknetMessage {
    pub call: Call,
    pub counterparty_height: Option<CosmosHeight>,
}

pub struct ProvideCallMessage;

impl<Chain: Async> ProvideMessageType<Chain> for ProvideCallMessage {
    type Message = StarknetMessage;
}
