use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::extract_data::MessageResponseExtractor;
use hermes_chain_components::traits::types::ibc::HasChannelIdType;
use hermes_chain_components::traits::types::ibc_events::channel::{
    ProvideChannelOpenInitEvent, ProvideChannelOpenTryEvent,
};
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;

use crate::impls::events::UseStarknetEvents;
use crate::impls::types::events::{StarknetChannelOpenInitEvent, StarknetChannelOpenTryEvent};
use crate::types::channel_id::ChannelId;
use crate::types::message_response::StarknetMessageResponse;

impl<Chain, Counterparty> ProvideChannelOpenInitEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasChannelIdType<Counterparty, ChannelId = ChannelId>,
{
    type ChannelOpenInitEvent = StarknetChannelOpenInitEvent;

    fn channel_open_init_event_channel_id(event: &StarknetChannelOpenInitEvent) -> &ChannelId {
        &event.channel_id
    }
}

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

impl<Chain, Counterparty> ProvideChannelOpenTryEvent<Chain, Counterparty> for UseStarknetEvents
where
    Chain: HasChannelIdType<Counterparty, ChannelId = ChannelId>,
{
    type ChannelOpenTryEvent = StarknetChannelOpenTryEvent;

    fn channel_open_try_event_channel_id(event: &StarknetChannelOpenTryEvent) -> &ChannelId {
        &event.channel_id
    }
}

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
