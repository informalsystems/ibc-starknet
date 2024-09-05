use cgp::prelude::*;
use hermes_cairo_encoding_components::components::encode_mut::*;
pub use hermes_cairo_encoding_components::components::encoding::*;
use hermes_cairo_encoding_components::impls::encode_mut::delegate::DelegateEncodeMutComponents;
use hermes_cairo_encoding_components::impls::encode_mut::option::EncodeOption;
use hermes_cairo_encoding_components::impls::encode_mut::pair::EncoderPair;
use hermes_cairo_encoding_components::impls::encode_mut::reference::EncodeDeref;
use hermes_cairo_encoding_components::impls::encode_mut::tagged::EncodeTagged;
use hermes_cairo_encoding_components::impls::encode_mut::vec::EncodeList;
use hermes_cairo_encoding_components::impls::encode_mut::with_context::EncodeWithContext;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::tagged::Tagged;
pub use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
pub use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use starknet::core::types::{Felt, U256};

use crate::types::messages::erc20::deploy::{
    DeployErc20TokenMessage, EncodeDeployErc20TokenMessage,
};
use crate::types::messages::erc20::transfer::{
    EncodeTransferErc20TokenMessage, TransferErc20TokenMessage,
};
use crate::types::messages::ibc::denom::{
    Denom, EncodeDenom, EncodePrefixedDenom, EncodeTracePrefix, PrefixedDenom, TracePrefix,
};
use crate::types::messages::ibc::height::{EncodeHeight, Height};
use crate::types::messages::ibc::ibc_transfer::{
    EncodeIbcTransferMessage, EncodeParticipant, IbcTransferMessage, Participant,
};
use crate::types::messages::ibc::packet::{EncodePacket, Packet};

define_components! {
    StarknetCairoEncodingComponents {
        [
            EncodedTypeComponent,
            EncodeBufferTypeComponent,
            DecodeBufferTypeComponent,
            DecodeBufferPeekerComponent,
            EncoderComponent,
            DecoderComponent,
        ]: CairoEncodingComponents,
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]:
            DelegateEncodeMutComponents<StarknetEncodeMutComponents>
    }
}

pub struct StarknetEncodeMutComponents;

with_cairo_encode_mut_components! {
    delegate_components! {
        StarknetEncodeMutComponents {
            @CairoEncodeMutComponents: DelegateEncodeMutComponents<CairoEncodeMutComponents>,
        }
    }
}

delegate_components! {
    StarknetEncodeMutComponents {
        <'a, V> (ViaCairo, &'a V): EncodeDeref,
        <V> (ViaCairo, Option<V>): EncodeOption<V>,
        <Tag, Value> (ViaCairo, Tagged<Tag, Value>): EncodeTagged,
        <A, B> (ViaCairo, (A, B)): EncoderPair<EncodeWithContext, EncodeWithContext>,
        (ViaCairo, TransferErc20TokenMessage): EncodeTransferErc20TokenMessage,
        (ViaCairo, DeployErc20TokenMessage): EncodeDeployErc20TokenMessage,
        (ViaCairo, Denom): EncodeDenom,
        (ViaCairo, TracePrefix): EncodeTracePrefix,
        (ViaCairo, Vec<TracePrefix>): EncodeList,
        (ViaCairo, PrefixedDenom): EncodePrefixedDenom,
        (ViaCairo, Participant): EncodeParticipant,
        (ViaCairo, IbcTransferMessage): EncodeIbcTransferMessage,
        (ViaCairo, Height): EncodeHeight,
        (ViaCairo, Packet): EncodePacket,
    }
}
