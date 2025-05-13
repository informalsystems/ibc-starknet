use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    ChannelOpenInitEventComponent, ChannelOpenTryEventComponent, HasChannelIdType,
    MessageResponseExtractor, MessageResponseExtractorComponent, ProvideChannelOpenInitEvent,
    ProvideChannelOpenTryEvent,
};
use hermes_core::chain_type_components::traits::HasMessageResponseType;
use hermes_core::encoding_components::traits::{CanDecode, HasDefaultEncoding, HasEncodedType};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::ChannelId;
use starknet::core::types::Felt;

use crate::impls::{StarknetChannelOpenInitEvent, StarknetChannelOpenTryEvent, UseStarknetEvents};
use crate::types::StarknetMessageResponse;

#[cgp_provider(ChannelOpenInitEventComponent)]
impl<Chain, Counterparty> ProvideChannelOpenInitEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasChannelIdType<Counterparty, ChannelId = ChannelId>,
{
    type ChannelOpenInitEvent = StarknetChannelOpenInitEvent;

    fn channel_open_init_event_channel_id(event: &StarknetChannelOpenInitEvent) -> &ChannelId {
        &event.channel_id
    }
}

#[cgp_provider(MessageResponseExtractorComponent)]
impl<Chain, Encoding> MessageResponseExtractor<Chain, StarknetChannelOpenInitEvent>
    for UseStarknetEvents
where
    Chain: HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, ChannelId>,
{
    fn try_extract_from_message_response(
        _chain: &Chain,
        _tag: PhantomData<StarknetChannelOpenInitEvent>,
        message_response: &StarknetMessageResponse,
    ) -> Option<StarknetChannelOpenInitEvent> {
        let channel_id = Chain::default_encoding()
            .decode(&message_response.result)
            .ok()?;

        Some(StarknetChannelOpenInitEvent { channel_id })
    }
}

#[cgp_provider(ChannelOpenTryEventComponent)]
impl<Chain, Counterparty> ProvideChannelOpenTryEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasChannelIdType<Counterparty, ChannelId = ChannelId>,
{
    type ChannelOpenTryEvent = StarknetChannelOpenTryEvent;

    fn channel_open_try_event_channel_id(event: &StarknetChannelOpenTryEvent) -> &ChannelId {
        &event.channel_id
    }
}

#[cgp_provider(MessageResponseExtractorComponent)]
impl<Chain, Encoding> MessageResponseExtractor<Chain, StarknetChannelOpenTryEvent>
    for UseStarknetEvents
where
    Chain: HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, ChannelId>,
{
    fn try_extract_from_message_response(
        _chain: &Chain,
        _tag: PhantomData<StarknetChannelOpenTryEvent>,
        message_response: &StarknetMessageResponse,
    ) -> Option<StarknetChannelOpenTryEvent> {
        let channel_id = Chain::default_encoding()
            .decode(&message_response.result)
            .ok()?;

        Some(StarknetChannelOpenTryEvent { channel_id })
    }
}
