use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;
use starknet::core::types::OrderedEvent;

pub struct ProvideStarknetEvent;

impl<Chain: Async> ProvideEventType<Chain> for ProvideStarknetEvent {
    type Event = OrderedEvent;
}
