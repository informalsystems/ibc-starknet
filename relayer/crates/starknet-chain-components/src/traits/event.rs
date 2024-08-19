use core::fmt::Debug;

use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::event::HasEventType;

#[derive_component(EventDecoderComponent, EventDecoder<Chain>)]
pub trait CanDecodeEvent<Event>: HasEventType + HasErrorType {
    fn decode_event(&self, event: &Self::Event) -> Result<Event, Self::Error>;
}

pub struct UnknownEvent<'a, Chain: HasEventType> {
    pub event: &'a Chain::Event,
}

pub struct EventSelectorMissing<'a, Chain: HasEventType> {
    pub event: &'a Chain::Event,
}

impl<'a, Chain> Debug for EventSelectorMissing<'a, Chain>
where
    Chain: HasEventType<Event: Debug>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "cannot parse event with missing selector: {:?}",
            self.event
        )
    }
}

impl<'a, Chain> Debug for UnknownEvent<'a, Chain>
where
    Chain: HasEventType<Event: Debug>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "failed to parse unknown event: {:?}", self.event)
    }
}
