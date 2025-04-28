use cgp::prelude::*;
use hermes_core::chain_components::traits::{
    CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
    ProvideCreateClientPayloadOptionsType, ProvideCreateClientPayloadType,
    ProvideUpdateClientPayloadType, UpdateClientPayloadTypeComponent,
};

use crate::types::payloads::client::{
    StarknetCreateClientPayload, StarknetCreateClientPayloadOptions, StarknetUpdateClientPayload,
};

pub struct ProvideStarknetPayloadTypes;

#[cgp_provider(CreateClientPayloadTypeComponent)]
impl<Chain: Async, Counterparty> ProvideCreateClientPayloadType<Chain, Counterparty>
    for ProvideStarknetPayloadTypes
{
    type CreateClientPayload = StarknetCreateClientPayload;
}

#[cgp_provider(CreateClientPayloadOptionsTypeComponent)]
impl<Chain: Async, Counterparty> ProvideCreateClientPayloadOptionsType<Chain, Counterparty>
    for ProvideStarknetPayloadTypes
{
    type CreateClientPayloadOptions = StarknetCreateClientPayloadOptions;
}

#[cgp_provider(UpdateClientPayloadTypeComponent)]
impl<Chain: Async, Counterparty> ProvideUpdateClientPayloadType<Chain, Counterparty>
    for ProvideStarknetPayloadTypes
{
    type UpdateClientPayload = StarknetUpdateClientPayload;
}
