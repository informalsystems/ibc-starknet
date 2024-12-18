use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::types::ibc::HasChannelIdType;
use hermes_chain_components::traits::types::ibc_events::channel::{
    ProvideChannelOpenInitEvent, ProvideChannelOpenTryEvent,
};
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::Felt;

use crate::types::channel_id::ChannelId;
use crate::types::message_response::StarknetMessageResponse;

pub struct UseStarknetChannelEvents;

impl<Chain, Counterparty, Encoding> ProvideChannelOpenInitEvent<Chain, Counterparty>
    for UseStarknetChannelEvents
where
    Chain: HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, ChannelId>,
{
    type ChannelOpenInitEvent = ChannelId;

    fn try_extract_channel_open_init_event(
        response: &StarknetMessageResponse,
    ) -> Option<ChannelId> {
        Chain::default_encoding().decode(&response.result).ok()
    }

    fn channel_open_init_event_channel_id(channel_id: &ChannelId) -> &ChannelId {
        channel_id
    }
}

impl<Chain, Counterparty, Encoding> ProvideChannelOpenTryEvent<Chain, Counterparty>
    for UseStarknetChannelEvents
where
    Chain: HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasDefaultEncoding<AsFelt, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, ChannelId>,
{
    type ChannelOpenTryEvent = ChannelId;

    fn try_extract_channel_open_try_event(response: &StarknetMessageResponse) -> Option<ChannelId> {
        Chain::default_encoding().decode(&response.result).ok()
    }

    fn channel_open_try_event_channel_id(channel_id: &ChannelId) -> &ChannelId {
        channel_id
    }
}
