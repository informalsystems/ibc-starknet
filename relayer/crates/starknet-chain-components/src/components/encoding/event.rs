use cgp::core::component::UseDelegate;
use cgp::prelude::*;
pub use hermes_cairo_encoding_components::components::encoding::EncodedTypeComponent;
use hermes_cairo_encoding_components::strategy::ViaCairo;
pub use hermes_encoding_components::traits::decode::DecoderComponent;
use hermes_encoding_components::traits::types::encoded::ProvideEncodedType;

use crate::impls::encoding::option::DecodeOptionalByClassHash;
use crate::types::event::StarknetEvent;
use crate::types::events::connection::{
    ConnOpenAckEvent, ConnectionHandshakeEvents, DecodeConnectionHandshakeEvents,
    InitConnectionEvent,
};
use crate::types::events::erc20::{ApprovalEvent, DecodeErc20Events, Erc20Event, TransferEvent};
use crate::types::events::ics20::{
    CreateIbcTokenEvent, DecodeIbcTransferEvents, IbcTransferEvent, ReceiveIbcTransferEvent,
};

cgp_preset! {
    StarknetEventEncodingComponents {
        EncodedTypeComponent: ProvideEncodedStarknetEventType,
        DecoderComponent: UseDelegate<StarknetEventEncoderComponents>,
    }
}

pub struct StarknetEventEncoderComponents;

pub struct ProvideEncodedStarknetEventType;

impl<Encoding: Async> ProvideEncodedType<Encoding> for ProvideEncodedStarknetEventType {
    type Encoded = StarknetEvent;
}

delegate_components! {
    StarknetEventEncoderComponents {
        [
            (ViaCairo, Erc20Event),
            (ViaCairo, TransferEvent),
            (ViaCairo, ApprovalEvent),
        ]:
            DecodeErc20Events,
        [
            (ViaCairo, IbcTransferEvent),
            (ViaCairo, ReceiveIbcTransferEvent),
            (ViaCairo, CreateIbcTokenEvent),
        ]:
            DecodeIbcTransferEvents,
        [
            (ViaCairo, ConnectionHandshakeEvents),
            (ViaCairo, InitConnectionEvent),
            (ViaCairo, ConnOpenAckEvent),
        ]:
            DecodeConnectionHandshakeEvents,
        (ViaCairo, Option<Erc20Event>):
            DecodeOptionalByClassHash<symbol!("erc20_hashes")>,
        (ViaCairo, Option<IbcTransferEvent>):
            DecodeOptionalByClassHash<symbol!("ics20_hashes")>,
        (ViaCairo, Option<ConnectionHandshakeEvents>):
            DecodeOptionalByClassHash<symbol!("ibc_core_hashes")>,
    }
}
