use ibc::core::client::types::Height as CosmosHeight;
use starknet_v13::core::types::Call;

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
