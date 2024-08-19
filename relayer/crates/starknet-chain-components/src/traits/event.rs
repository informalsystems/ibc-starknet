use cgp_core::prelude::*;
use hermes_encoding_components::traits::encoded::HasEncodedType;

use crate::traits::types::method::HasSelectorType;

#[derive_component(StarknetEventDecoderComponent, StarknetEventDecoder<Encoding>)]
pub trait CanDecodeStarknetEvent<Event>: HasSelectorType + HasEncodedType + HasErrorType {
    fn decode_event(
        &self,
        selector: &Self::Selector,
        keys: &Self::Encoded,
        values: &Self::Encoded,
    ) -> Result<Event, Self::Error>;
}

// #[derive(Debug)]
// pub struct UnknownEvent<Encoding: HasSelectorType>;
