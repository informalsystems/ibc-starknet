use core::ops::Deref;
use std::sync::Arc;

use starknet::core::types::{EmittedEvent, Felt, OrderedEvent};

use crate::impls::types::address::StarknetAddress;

#[derive(Debug, Clone)]
pub struct StarknetEvent {
    pub fields: Arc<StarknetEventFields>,
}

#[derive(Debug)]
pub struct StarknetEventFields {
    pub contract_address: StarknetAddress,
    pub class_hash: Option<Felt>,
    pub selector: Option<Felt>,
    pub keys: Vec<Felt>,
    pub data: Vec<Felt>,
}

impl Deref for StarknetEvent {
    type Target = StarknetEventFields;

    fn deref(&self) -> &StarknetEventFields {
        &self.fields
    }
}

#[derive(Debug)]
pub struct UnknownEvent<'a> {
    pub event: &'a StarknetEvent,
}

impl StarknetEvent {
    pub fn from_ordered_event(
        contract_address: StarknetAddress,
        class_hash: Felt,
        event: OrderedEvent,
    ) -> Self {
        let mut keys = event.keys.into_iter();
        let data = event.data;

        let selector = keys.next();

        Self {
            fields: Arc::new(StarknetEventFields {
                contract_address,
                class_hash: Some(class_hash),
                selector,
                keys: keys.collect(),
                data,
            }),
        }
    }
}

impl From<EmittedEvent> for StarknetEvent {
    fn from(event: EmittedEvent) -> Self {
        let mut keys = event.keys.into_iter();
        let selector = keys.next();

        Self {
            fields: Arc::new(StarknetEventFields {
                contract_address: event.from_address.into(),
                class_hash: None,
                selector,
                keys: keys.collect(),
                data: event.data,
            }),
        }
    }
}
