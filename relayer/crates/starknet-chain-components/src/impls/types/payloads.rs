use hermes_core::chain_components::traits::{
    CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
    ProvideCreateClientPayloadOptionsType, ProvideCreateClientPayloadType,
    ProvideRecoverClientPayloadType, ProvideUpdateClientPayloadType,
    RecoverClientPayloadTypeComponent, UpdateClientPayloadTypeComponent,
};
use hermes_prelude::*;

use crate::impls::StarknetRecoverClientPayload;
use crate::types::{
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

#[cgp_provider(RecoverClientPayloadTypeComponent)]
impl<Chain: Async> ProvideRecoverClientPayloadType<Chain> for ProvideStarknetPayloadTypes {
    type RecoverClientPayload = StarknetRecoverClientPayload;
}
