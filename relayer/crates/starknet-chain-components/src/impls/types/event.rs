use cgp::prelude::*;
use hermes_cosmos_chain_components::components::client::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;

use crate::types::event::StarknetEvent;

pub struct ProvideStarknetEvent;

#[cgp_provider(EventTypeComponent)]
impl<Chain: Async> ProvideEventType<Chain> for ProvideStarknetEvent {
    type Event = StarknetEvent;
}
