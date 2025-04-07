use ibc::core::client::types::Height as CosmosHeight;
use starknet::core::types::Felt;

#[derive(Clone)]
pub struct StarknetMessage {
    pub to: Felt,
    pub selector: Felt,
    pub calldata: Vec<Felt>,
    pub counterparty_height: Option<CosmosHeight>,
}

impl StarknetMessage {
    pub fn new(to: Felt, selector: Felt, calldata: Vec<Felt>) -> Self {
        Self {
            to,
            selector,
            calldata,
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
