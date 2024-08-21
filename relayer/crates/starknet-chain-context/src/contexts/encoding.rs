use core::iter::Peekable;
use core::slice::Iter;

use cgp_core::error::{DelegateErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_cairo_encoding_components::components::encode_mut::*;
use hermes_cairo_encoding_components::components::encoding::*;
use hermes_cairo_encoding_components::impls::encode_mut::delegate::DelegateEncodeMutComponents;
use hermes_cairo_encoding_components::impls::encode_mut::option::EncodeOption;
use hermes_cairo_encoding_components::impls::encode_mut::pair::{EncodeCons, EncoderPair};
use hermes_cairo_encoding_components::impls::encode_mut::reference::EncodeDeref;
use hermes_cairo_encoding_components::impls::encode_mut::tagged::EncodeTagged;
use hermes_cairo_encoding_components::impls::encode_mut::vec::EncodeList;
use hermes_cairo_encoding_components::impls::encode_mut::with_context::EncodeWithContext;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::traits::decode_mut::{
    CanPeekDecodeBuffer, HasDecodeBufferType, MutDecoderComponent,
};
use hermes_cairo_encoding_components::traits::encode_and_decode_mut::MutEncoderAndDecoder;
use hermes_cairo_encoding_components::traits::encode_mut::{
    HasEncodeBufferType, MutEncoderComponent,
};
use hermes_cairo_encoding_components::types::tagged::Tagged;
use hermes_cairo_encoding_components::HList;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::encoder::CanEncode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, HasEncodingType, ProvideEncodingType,
};
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::HermesError;
use hermes_starknet_chain_components::types::messages::erc20::deploy::{
    DeployErc20TokenMessage, EncodeDeployErc20TokenMessage,
};
use hermes_starknet_chain_components::types::messages::erc20::transfer::{
    EncodeTransferErc20TokenMessage, TransferErc20TokenMessage,
};
use hermes_starknet_chain_components::types::messages::ibc::denom::{
    Denom, EncodeDenom, EncodePrefixedDenom, EncodeTracePrefix, PrefixedDenom, TracePrefix,
};
use hermes_starknet_chain_components::types::messages::ibc::height::{EncodeHeight, Height};
use hermes_starknet_chain_components::types::messages::ibc::ibc_transfer::{
    EncodeIbcTransferMessage, EncodeParticipant, IbcTransferMessage, Participant,
};
use hermes_starknet_chain_components::types::messages::ibc::packet::{EncodePacket, Packet};
use starknet::core::types::{Felt, U256};

use crate::impls::error::HandleStarknetError;

pub struct CairoEncoding;

pub struct CairoEncodingContextComponents;

pub struct StarknetEncodeMutComponents;

impl HasComponents for CairoEncoding {
    type Components = CairoEncodingContextComponents;
}

delegate_components! {
    CairoEncodingContextComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DelegateErrorRaiser<HandleStarknetError>,
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]:
            DelegateEncodeMutComponents<StarknetEncodeMutComponents>,
    }
}

with_cairo_encoding_components! {
    delegate_components! {
        CairoEncodingContextComponents {
            @CairoEncodingComponents: CairoEncodingComponents,
        }
    }
}

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

pub struct ProvideCairoEncoding;

delegate_components! {
    ProvideCairoEncoding {
        EncodingGetterComponent: GetDefaultEncoding,
    }
}

impl<Context> ProvideEncodingType<Context> for ProvideCairoEncoding
where
    Context: Async,
{
    type Encoding = CairoEncoding;
}

impl<Context> DefaultEncodingGetter<Context> for ProvideCairoEncoding
where
    Context: HasEncodingType<Encoding = CairoEncoding>,
{
    fn default_encoding() -> &'static CairoEncoding {
        &CairoEncoding
    }
}

pub trait CanUseCairoEncoding:
    HasErrorType<Error = HermesError>
    + HasEncodedType<Encoded = Vec<Felt>>
    + HasEncodeBufferType<EncodeBuffer = Vec<Felt>>
    + for<'a> HasDecodeBufferType<DecodeBuffer<'a> = Peekable<Iter<'a, Felt>>>
    + CanPeekDecodeBuffer<Felt>
    + CanEncodeAndDecode<ViaCairo, ()>
    + CanEncodeAndDecode<ViaCairo, Felt>
    + CanEncodeAndDecode<ViaCairo, Felt>
    + CanEncodeAndDecode<ViaCairo, u128>
    + CanEncodeAndDecode<ViaCairo, U256>
    + CanEncodeAndDecode<ViaCairo, u64>
    + CanEncodeAndDecode<ViaCairo, usize>
    + CanEncodeAndDecode<ViaCairo, Vec<u8>>
    + CanEncodeAndDecode<ViaCairo, Vec<Felt>>
    + CanEncodeAndDecode<ViaCairo, String>
    + CanEncode<ViaCairo, TransferErc20TokenMessage>
    + CanEncode<ViaCairo, DeployErc20TokenMessage>
    + CanEncodeAndDecode<ViaCairo, Option<String>>
    + for<'a> CanEncode<ViaCairo, &'a String>
    + CanEncodeAndDecode<ViaCairo, Denom>
    + CanEncodeAndDecode<ViaCairo, PrefixedDenom>
    + CanEncodeAndDecode<ViaCairo, TracePrefix>
    + CanEncodeAndDecode<ViaCairo, Vec<TracePrefix>>
    + CanEncodeAndDecode<ViaCairo, Participant>
    + CanEncode<ViaCairo, IbcTransferMessage>
    + CanEncodeAndDecode<ViaCairo, Height>
    + CanEncodeAndDecode<ViaCairo, Packet>
    + CanEncode<ViaCairo, HList![String, String, String]>
{
}

impl CanUseCairoEncoding for CairoEncoding {}

pub trait CanUsePairEncoder:
    MutEncoderAndDecoder<CairoEncoding, ViaCairo, (u128, (Vec<u8>, U256))>
{
}

impl CanUsePairEncoder for EncodeCons<EncodeCons<EncodeWithContext>> {}
