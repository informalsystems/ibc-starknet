use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::message::ProvideMessageType;
use ibc::core::client::types::Height as CosmosHeight;
use starknet::accounts::Call;

#[derive(Clone)]
pub struct StarknetMessage {
    pub call: Call,
    pub counterparty_height: Option<CosmosHeight>,
}

impl StarknetMessage {
    pub fn new(call: Call) -> Self {
        Self {
            call,
            counterparty_height: None,
        }
    }

    pub fn with_counterparty_height(mut self, height: CosmosHeight) -> Self {
        self.counterparty_height = Some(height);
        self
    }

    pub fn without_counterparty_height(mut self) -> Self {
        self.counterparty_height = None;
        self
    }
}

pub struct ProvideCallMessage;

impl<Chain: Async> ProvideMessageType<Chain> for ProvideCallMessage {
    type Message = StarknetMessage;
}
