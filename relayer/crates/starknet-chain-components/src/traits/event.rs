use core::fmt::Debug;
use std::marker::PhantomData;

use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::event::HasEventType;

#[derive_component(EventParserComponent, EventParser<Chain>)]
pub trait CanParseEvent<Event>: HasEventType + HasErrorType {
    fn parse_event(&self, event: &Self::Event) -> Result<Event, Self::Error>;
}

pub struct UnknownEvent<'a, Chain: HasEventType> {
    pub event: &'a Chain::Event,
}

pub struct EventSelectorMissing<'a, Chain: HasEventType> {
    pub event: &'a Chain::Event,
}

pub struct DelegateEventDecoders<Components>(pub PhantomData<Components>);

pub trait CanParseEvents<Event>: HasEventType + HasErrorType {
    fn parse_events(&self, events: &[Self::Event]) -> Result<Vec<Event>, Self::Error>;
}

impl<Chain, Event> CanParseEvents<Event> for Chain
where
    Chain: CanParseEvent<Event>,
{
    fn parse_events(&self, events: &[Self::Event]) -> Result<Vec<Event>, Self::Error> {
        let mut parsed_events = Vec::new();

        for event in events.iter() {
            let parsed_event = self.parse_event(event)?;
            parsed_events.push(parsed_event);
        }

        Ok(parsed_events)
    }
}

impl<Chain, Components, Event> EventParser<Chain, Event> for DelegateEventDecoders<Components>
where
    Chain: HasEventType + HasErrorType,
    Components: DelegateComponent<Event>,
    Components::Delegate: EventParser<Chain, Event>,
{
    fn parse_event(chain: &Chain, event: &Chain::Event) -> Result<Event, Chain::Error> {
        Components::Delegate::parse_event(chain, event)
    }
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
