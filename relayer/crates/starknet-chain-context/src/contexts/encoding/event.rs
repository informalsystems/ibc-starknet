use std::collections::HashSet;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent, HasEncoding,
};
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_error::impls::ProvideHermesError;
use hermes_starknet_chain_components::components::encoding::event::*;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::types::event::StarknetEvent;
use hermes_starknet_chain_components::types::events::channel::ChannelHandshakeEvents;
use hermes_starknet_chain_components::types::events::connection::ConnectionHandshakeEvents;
use hermes_starknet_chain_components::types::events::erc20::Erc20Event;
use hermes_starknet_chain_components::types::events::ics20::IbcTransferEvent;
use hermes_starknet_chain_components::types::events::packet::PacketRelayEvents;
use starknet::core::types::Felt;

use crate::contexts::encoding::cairo::{ProvideCairoEncoding, StarknetCairoEncoding};
use crate::impls::error::HandleStarknetChainError;

#[cgp_context(StarknetEventEncodingContextComponents: StarknetEventEncodingComponents)]
#[derive(HasField, Default, Clone)]
pub struct StarknetEventEncoding {
    pub erc20_hashes: HashSet<Felt>,
    pub ics20_hashes: HashSet<Felt>,
    pub ibc_client_hashes: HashSet<Felt>,
    pub ibc_core_contract_addresses: HashSet<StarknetAddress>,
}

delegate_components! {
    StarknetEventEncodingContextComponents{
        ErrorTypeProviderComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        [
            EncodingTypeComponent,
            EncodingGetterComponent,
            DefaultEncodingGetterComponent
        ]:
            ProvideCairoEncoding,
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
