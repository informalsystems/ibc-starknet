use cgp::prelude::HasAsyncErrorType;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_core::encoding_components::traits::{CanDecode, HasEncodedType};

pub trait CanFilterDecodeEvents<Value>: HasEncodedType + HasAsyncErrorType {
    fn filter_decode_events(&self, encoded: &[Self::Encoded]) -> Result<Vec<Value>, Self::Error>;
}

impl<Encoding, Value> CanFilterDecodeEvents<Value> for Encoding
where
    Encoding: CanDecode<ViaCairo, Option<Value>>,
{
    fn filter_decode_events(
        &self,
        raw_events: &[Self::Encoded],
    ) -> Result<Vec<Value>, Self::Error> {
        let mut events = Vec::new();

        for raw_event in raw_events {
            if let Ok(Some(event)) = self.decode(raw_event) {
                events.push(event);
            }
        }

        Ok(events)
    }
}
