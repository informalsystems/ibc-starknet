use core::slice::Iter;

use cgp_core::error::{DelegateErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_cairo_encoding_components::components::encode_mut::CairoEncodeMutComponents;
use hermes_cairo_encoding_components::components::encoding::*;
use hermes_cairo_encoding_components::impls::encode_mut::delegate::DelegateEncodeMutComponents;
use hermes_cairo_encoding_components::impls::encode_mut::pair::EncodeCons;
use hermes_cairo_encoding_components::impls::encode_mut::with_context::EncodeWithContext;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::traits::decode_mut::{
    HasDecodeBufferType, MutDecoderComponent,
};
use hermes_cairo_encoding_components::traits::encode_and_decode_mut::MutEncoderAndDecoder;
use hermes_cairo_encoding_components::traits::encode_mut::{
    HasEncodeBufferType, MutEncoderComponent,
};
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::HermesError;
use starknet::core::types::{Felt, U256};

use crate::impls::error::HandleStarknetError;

pub struct CairoEncoding;

pub struct CairoEncodingContextComponents;

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
            DelegateEncodeMutComponents<CairoEncodeMutComponents>,
    }
}

with_cairo_encoding_components! {
    delegate_components! {
        CairoEncodingContextComponents {
            @CairoEncodingComponents: CairoEncodingComponents,
        }
    }
}

pub trait CanUseCairoEncoding:
    HasErrorType<Error = HermesError>
    + HasEncodedType<Encoded = Vec<Felt>>
    + HasEncodeBufferType<EncodeBuffer = Vec<Felt>>
    + for<'a> HasDecodeBufferType<DecodeBuffer<'a> = Iter<'a, Felt>>
    + CanEncodeAndDecode<ViaCairo, Felt>
    + CanEncodeAndDecode<ViaCairo, Felt>
    + CanEncodeAndDecode<ViaCairo, u128>
    + CanEncodeAndDecode<ViaCairo, U256>
    + CanEncodeAndDecode<ViaCairo, u64>
    + CanEncodeAndDecode<ViaCairo, usize>
    + CanEncodeAndDecode<ViaCairo, Vec<u8>>
    + CanEncodeAndDecode<ViaCairo, String>
{
}

impl CanUseCairoEncoding for CairoEncoding {}

pub trait CanUsePairEncoder:
    MutEncoderAndDecoder<CairoEncoding, ViaCairo, (u128, (Vec<u8>, U256))>
{
}

impl CanUsePairEncoder for EncodeCons<EncodeCons<EncodeWithContext>> {}
