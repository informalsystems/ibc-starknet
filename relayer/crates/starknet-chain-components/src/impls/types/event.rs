use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;

pub struct ProvideDummyEvent;

impl<Chain: Async> ProvideEventType<Chain> for ProvideDummyEvent {
    type Event = ();
}
