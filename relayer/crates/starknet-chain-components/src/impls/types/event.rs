use cgp::prelude::*;
use hermes_chain_components::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;

use crate::types::event::StarknetEvent;

pub struct ProvideStarknetEvent;

#[cgp_provider(EventTypeComponent)]
impl<Chain: Async> ProvideEventType<Chain> for ProvideStarknetEvent {
    type Event = StarknetEvent;
}
