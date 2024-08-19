use starknet::core::types::{Felt, OrderedEvent};

#[derive(Debug)]
pub struct StarknetEvent {
    pub selector: Option<Felt>,
    pub keys: Vec<Felt>,
    pub data: Vec<Felt>,
}

impl From<OrderedEvent> for StarknetEvent {
    fn from(event: OrderedEvent) -> Self {
        let mut keys = event.keys.into_iter();
        let data = event.data;

        let selector = keys.next();

        Self {
            selector,
            keys: keys.collect(),
            data,
        }
    }
}
