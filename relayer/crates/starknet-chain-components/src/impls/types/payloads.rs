use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::create_client::{
    ProvideCreateClientPayloadOptionsType, ProvideCreateClientPayloadType,
};

use crate::types::payloads::client::{
    StarknetCreateClientPayload, StarknetCreateClientPayloadOptions,
};

pub struct ProvideStarknetPayloadTypes;

impl<Chain: Async, Counterparty> ProvideCreateClientPayloadType<Chain, Counterparty>
    for ProvideStarknetPayloadTypes
{
    type CreateClientPayload = StarknetCreateClientPayload;
}

impl<Chain: Async, Counterparty> ProvideCreateClientPayloadOptionsType<Chain, Counterparty>
    for ProvideStarknetPayloadTypes
{
    type CreateClientPayloadOptions = StarknetCreateClientPayloadOptions;
}
