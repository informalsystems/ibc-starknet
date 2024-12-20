use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::ibc_events::write_ack::ProvideWriteAckEvent;
use hermes_chain_components::traits::types::packets::ack::HasAcknowledgementType;

use crate::types::event::StarknetEvent;

pub struct UseStarknetWriteAckEvent;

impl<Chain, Counterparty> ProvideWriteAckEvent<Chain, Counterparty> for UseStarknetWriteAckEvent
where
    Chain: HasEventType<Event = StarknetEvent>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>,
{
    type WriteAckEvent = Vec<u8>;

    fn try_extract_write_ack_event(_event: &StarknetEvent) -> Option<Self::WriteAckEvent> {
        todo!()
    }

    fn write_acknowledgement(ack: &Vec<u8>) -> impl AsRef<Vec<u8>> + Send {
        ack
    }
}
