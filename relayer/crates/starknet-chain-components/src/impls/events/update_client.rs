use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_core::chain_components::traits::{
    EventExtractor, EventExtractorComponent, HasEventType, ProvideUpdateClientEvent,
    UpdateClientEventComponent,
};
use hermes_core::encoding_components::traits::{CanDecode, HasEncodedType, HasEncoding};
use hermes_prelude::*;

use crate::impls::{StarknetUpdateClientEvent, UseStarknetEvents};
use crate::types::PacketRelayEvents;

#[cgp_provider(UpdateClientEventComponent)]
impl<Chain> ProvideUpdateClientEvent<Chain> for UseStarknetEvents {
    type UpdateClientEvent = StarknetUpdateClientEvent;
}

#[cgp_provider(EventExtractorComponent)]
impl<Chain, Encoding> EventExtractor<Chain, StarknetUpdateClientEvent> for UseStarknetEvents
where
    Chain: HasEventType + HasEncoding<AsStarknetEvent, Encoding = Encoding>,
    Encoding:
        HasEncodedType<Encoded = Chain::Event> + CanDecode<ViaCairo, Option<PacketRelayEvents>>,
{
    fn try_extract_from_event(
        chain: &Chain,
        _tag: PhantomData<StarknetUpdateClientEvent>,
        event: &Chain::Event,
    ) -> Option<StarknetUpdateClientEvent> {
        let event = chain.encoding().decode(event).ok()??;

        match event {
            PacketRelayEvents::UpdateClient(event) => Some(event),
            _ => None,
        }
    }
}
