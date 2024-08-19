use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::event::HasEventType;

#[derive_component(EventDecoderComponent, EventDecoder<Chain>)]
pub trait CanDecodeEvent<Event>: HasEventType + HasErrorType {
    fn decode_event(&self, event: &Self::Event) -> Result<Event, Self::Error>;
}

#[derive(Debug)]
pub struct UnknownEvent<'a, Chain: HasEventType> {
    pub event: &'a Chain::Event,
}

#[derive(Debug)]
pub struct EventSelectorMissing<'a, Chain: HasEventType> {
    pub event: &'a Chain::Event,
}
