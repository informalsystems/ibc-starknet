use cgp::prelude::*;
use hermes_cosmos_chain_components::components::client::{
    CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
    UpdateClientPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    ProvideCreateClientPayloadOptionsType, ProvideCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::update_client::ProvideUpdateClientPayloadType;

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
