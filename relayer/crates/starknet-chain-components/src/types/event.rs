use starknet::core::types::{EmittedEvent, Felt, OrderedEvent};

#[derive(Debug)]
pub struct StarknetEvent {
    pub contract_address: Felt,
    pub class_hash: Option<Felt>,
    pub selector: Option<Felt>,
    pub keys: Vec<Felt>,
    pub data: Vec<Felt>,
}

#[derive(Debug)]
pub struct UnknownEvent<'a> {
    pub event: &'a StarknetEvent,
}

impl StarknetEvent {
    pub fn from_ordered_event(
        contract_address: Felt,
        class_hash: Felt,
        event: OrderedEvent,
    ) -> Self {
        let mut keys = event.keys.into_iter();
        let data = event.data;

        let selector = keys.next();

        Self {
            contract_address,
            class_hash: Some(class_hash),
            selector,
            keys: keys.collect(),
            data,
        }
    }

    pub fn from_emitted_event(event: EmittedEvent) -> Self {
        let mut keys = event.keys.into_iter();
        let selector = keys.next();

        Self {
            contract_address: event.from_address,
            class_hash: None,
            selector,
            keys: keys.collect(),
            data: event.data,
        }
    }
}
