use cgp::prelude::*;
pub use hermes_cairo_encoding_components::components::encoding::EncodedTypeComponent;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
pub use hermes_encoding_components::traits::decode::DecoderComponent;
use hermes_encoding_components::traits::types::encoded::ProvideEncodedType;

use crate::types::event::StarknetEvent;
use crate::types::events::erc20::{ApprovalEvent, DecodeErc20Events, Erc20Event, TransferEvent};
use crate::types::events::ics20::{
    CreateIbcTokenEvent, DecodeIbcTransferEvents, IbcTransferEvent, ReceiveIbcTransferEvent,
};

define_components! {
    StarknetEventEncodingComponents {
        EncodedTypeComponent: ProvideEncodedStarknetEventType,
        DecoderComponent: DelegateEncoding<StarknetEventEncoderComponents>,
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
    }
}
