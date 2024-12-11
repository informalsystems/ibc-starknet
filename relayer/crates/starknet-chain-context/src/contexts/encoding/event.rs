use std::collections::HashSet;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
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
use hermes_starknet_chain_components::types::event::StarknetEvent;
use hermes_starknet_chain_components::types::events::channel::ChannelHandshakeEvents;
use hermes_starknet_chain_components::types::events::connection::ConnectionHandshakeEvents;
use hermes_starknet_chain_components::types::events::erc20::Erc20Event;
use hermes_starknet_chain_components::types::events::ics20::IbcTransferEvent;
use starknet::core::types::Felt;

use crate::contexts::encoding::cairo::{ProvideCairoEncoding, StarknetCairoEncoding};
use crate::impls::error::HandleStarknetChainError;

#[derive(HasField)]
pub struct StarknetEventEncoding {
    pub erc20_hashes: HashSet<Felt>,
    pub ics20_hashes: HashSet<Felt>,
    pub ibc_client_hashes: HashSet<Felt>,
    pub ibc_core_hashes: HashSet<Felt>,
}

pub struct StarknetEventEncodingContextComponents;

impl HasComponents for StarknetEventEncoding {
    type Components = StarknetEventEncodingContextComponents;
}

with_starknet_event_encoding_components! {
    | Components | {
        delegate_components! {
            StarknetEventEncodingContextComponents{
                Components: StarknetEventEncodingComponents,
            }
        }
    }
}

delegate_components! {
    StarknetEventEncodingContextComponents{
        ErrorTypeComponent: ProvideHermesError,
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
{
}

impl CanUseStarknetEventEncoding for StarknetEventEncoding {}
