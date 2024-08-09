use cgp_core::prelude::*;

pub struct CairoEncoding;

pub struct CairoEncodingComponents;

impl HasComponents for CairoEncoding {
    type Components = CairoEncodingComponents;
}

delegate_components! {
    CairoEncodingComponents {

    }
}
