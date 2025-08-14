use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_core::chain_components::traits::{
    EventExtractor, EventExtractorComponent, HasClientIdType, HasEventType, HasUpdateClientEvent,
    ProvideUpdateClientEvent, ProvideUpdateClientEventFields, UpdateClientEventComponent,
};
use hermes_core::encoding_components::traits::{CanDecode, HasEncodedType, HasEncoding};
use hermes_prelude::*;

use crate::components::StarknetChainComponents::re_exports::UpdateClientEventFieldsComponent;
use crate::impls::{StarknetUpdateClientEvent, UseStarknetEvents};
use crate::types::{ClientId, ClientRelayEvents};

#[cgp_provider(UpdateClientEventComponent)]
impl<Chain> ProvideUpdateClientEvent<Chain> for UseStarknetEvents {
    type UpdateClientEvent = StarknetUpdateClientEvent;
}

#[cgp_provider(UpdateClientEventFieldsComponent)]
impl<Chain, Counterparty> ProvideUpdateClientEventFields<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasUpdateClientEvent<UpdateClientEvent = StarknetUpdateClientEvent>
        + HasClientIdType<Counterparty, ClientId = ClientId>,
{
    fn client_id(chain: &Chain, event: &StarknetUpdateClientEvent) -> ClientId {
        event.client_id.clone()
    }
}

#[cgp_provider(EventExtractorComponent)]
impl<Chain, Encoding> EventExtractor<Chain, StarknetUpdateClientEvent> for UseStarknetEvents
where
    Chain: HasEventType + HasEncoding<AsStarknetEvent, Encoding = Encoding>,
    Encoding:
        HasEncodedType<Encoded = Chain::Event> + CanDecode<ViaCairo, Option<ClientRelayEvents>>,
{
    fn try_extract_from_event(
        chain: &Chain,
        _tag: PhantomData<StarknetUpdateClientEvent>,
        event: &Chain::Event,
    ) -> Option<StarknetUpdateClientEvent> {
        let event = chain.encoding().decode(event).ok()??;

        match event {
            ClientRelayEvents::UpdateClient(event) => Some(event),
        }
    }
}
