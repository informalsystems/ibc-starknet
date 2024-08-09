use core::slice::Iter;

use cgp_core::error::{DelegateErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_cairo_encoding_components::components::encoding::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::traits::decode_mut::HasDecodeBufferType;
use hermes_cairo_encoding_components::traits::encode_and_decode_mut::CanEncodeAndDecodeMut;
use hermes_cairo_encoding_components::traits::encode_mut::HasEncodeBufferType;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::HermesError;
use starknet::core::types::Felt;

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
    + HasEncodeBufferType<EncodeBuffer = Vec<Felt>>
    + for<'a> HasDecodeBufferType<DecodeBuffer<'a> = Iter<'a, Felt>>
    + CanEncodeAndDecodeMut<ViaCairo, Felt>
{
}

impl CanUseCairoEncoding for CairoEncoding {}
