use cgp_core::error::{DelegateErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_cairo_encoding_components::components::encoding::*;
use hermes_cairo_encoding_components::traits::encode_mut::CanEncodeMut;
use hermes_error::impls::ProvideHermesError;
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

pub trait CanUseCairoEncoding: CanEncodeMut<(), Felt> {}
