use core::iter::Peekable;
use core::slice::Iter;

use cgp_core::error::{DelegateErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::pair::EncodeCons;
use hermes_cairo_encoding_components::impls::encode_mut::with_context::EncodeWithContext;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_cairo_encoding_components::HList;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::decode_mut::CanPeekDecodeBuffer;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::encode_and_decode_mut::MutEncoderAndDecoder;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, HasEncodingType, ProvideEncodingType,
};
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::HermesError;
use hermes_starknet_chain_components::components::cairo_encoding::*;
use hermes_starknet_chain_components::types::messages::erc20::deploy::DeployErc20TokenMessage;
use hermes_starknet_chain_components::types::messages::erc20::transfer::TransferErc20TokenMessage;
use hermes_starknet_chain_components::types::messages::ibc::denom::{
    Denom, PrefixedDenom, TracePrefix,
};
use hermes_starknet_chain_components::types::messages::ibc::height::Height;
use hermes_starknet_chain_components::types::messages::ibc::ibc_transfer::{
    IbcTransferMessage, Participant,
};
use hermes_starknet_chain_components::types::messages::ibc::packet::Packet;
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
    }
}

hermes_starknet_chain_components::with_starknet_cairo_encoding_components! {
    delegate_components! {
        CairoEncodingContextComponents {
            @StarknetCairoEncodingComponents: StarknetCairoEncodingComponents,
        }
    }
}

pub struct ProvideCairoEncoding;

delegate_components! {
    ProvideCairoEncoding {
        EncodingGetterComponent: GetDefaultEncoding,
    }
}

impl<Context> ProvideEncodingType<Context, AsFelt> for ProvideCairoEncoding
where
    Context: Async,
{
    type Encoding = CairoEncoding;
}

impl<Context> DefaultEncodingGetter<Context, AsFelt> for ProvideCairoEncoding
where
    Context: HasEncodingType<AsFelt, Encoding = CairoEncoding>,
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
