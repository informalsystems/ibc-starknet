use std::collections::HashSet;
use std::sync::OnceLock;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::encoding_components::traits::{
    CanDecode, DefaultEncodingGetterComponent, EncodingGetterComponent,
    EncodingTypeProviderComponent, HasEncodedType, HasEncoding,
};
use hermes_cosmos::error::impls::UseHermesError;
use hermes_prelude::*;
use hermes_starknet_chain_components::components::*;
use hermes_starknet_chain_components::impls::StarknetAddress;
use hermes_starknet_chain_components::types::{
    ChannelHandshakeEvents, ConnectionHandshakeEvents, Erc20Event, IbcTransferEvent,
    PacketRelayEvents, StarknetEvent,
};
use starknet::core::types::Felt;

use crate::contexts::encoding::cairo::{StarknetCairoEncoding, UseStarknetCairoEncoding};
use crate::impls::error::HandleStarknetChainError;

#[cgp_context(StarknetEventEncodingContextComponents: StarknetEventEncodingComponents)]
#[derive(HasField, Default, Clone)]
pub struct StarknetEventEncoding {
    pub erc20_hashes: OnceLock<HashSet<Felt>>,
    pub ics20_hashes: OnceLock<HashSet<Felt>>,
    pub ibc_client_hashes: OnceLock<HashSet<Felt>>,
    pub ibc_core_contract_addresses: OnceLock<HashSet<StarknetAddress>>,
}

delegate_components! {
    StarknetEventEncodingContextComponents{
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        [
            EncodingTypeProviderComponent<AsFelt>,
            EncodingGetterComponent<AsFelt>,
            DefaultEncodingGetterComponent<AsFelt>,
        ]:
            UseStarknetCairoEncoding,
    }
}

pub trait CanUseStarknetEventEncoding:
    HasEncodedType<Encoded = StarknetEvent>
    + HasEncoding<AsFelt, Encoding = StarknetCairoEncoding>
    + CanDecode<ViaCairo, Erc20Event>
    + CanDecode<ViaCairo, IbcTransferEvent>
    + CanDecode<ViaCairo, Option<Erc20Event>>
    + CanDecode<ViaCairo, Option<IbcTransferEvent>>
    + CanDecode<ViaCairo, ConnectionHandshakeEvents>
    + CanDecode<ViaCairo, Option<ConnectionHandshakeEvents>>
    + CanDecode<ViaCairo, ChannelHandshakeEvents>
    + CanDecode<ViaCairo, Option<ChannelHandshakeEvents>>
    + CanDecode<ViaCairo, PacketRelayEvents>
    + CanDecode<ViaCairo, Option<PacketRelayEvents>>
{
}

impl CanUseStarknetEventEncoding for StarknetEventEncoding {}
