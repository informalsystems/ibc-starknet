use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::create_client::{
    ProvideCreateClientPayloadOptionsType, ProvideCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::update_client::ProvideUpdateClientPayloadType;

use crate::types::payloads::client::{
    StarknetCreateClientPayload, StarknetCreateClientPayloadOptions, StarknetUpdateClientPayload,
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

impl<Chain: Async, Counterparty> ProvideUpdateClientPayloadType<Chain, Counterparty>
    for ProvideStarknetPayloadTypes
{
    type UpdateClientPayload = StarknetUpdateClientPayload;
}
