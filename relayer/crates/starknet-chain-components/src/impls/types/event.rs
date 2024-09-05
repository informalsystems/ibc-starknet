use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;

use crate::types::event::StarknetEvent;

pub struct ProvideStarknetEvent;

impl<Chain: Async> ProvideEventType<Chain> for ProvideStarknetEvent {
    type Event = StarknetEvent;
}
