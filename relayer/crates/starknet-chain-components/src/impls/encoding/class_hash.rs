use core::marker::PhantomData;
use std::collections::HashSet;
use std::sync::OnceLock;

use hermes_core::encoding_components::traits::{CanDecode, Decoder, DecoderComponent};
use hermes_prelude::*;
use starknet::core::types::Felt;

use crate::types::event::StarknetEvent;

pub struct DecodeOptionalByClassHash<Tag>(pub PhantomData<Tag>);

#[cgp_provider(DecoderComponent)]
impl<Encoding, Strategy, Value, Tag> Decoder<Encoding, Strategy, Option<Value>>
    for DecodeOptionalByClassHash<Tag>
where
    Encoding: CanDecode<Strategy, Value, Encoded = StarknetEvent>
        + HasField<Tag, Value = OnceLock<HashSet<Felt>>>,
{
    fn decode(
        encoding: &Encoding,
        event: &StarknetEvent,
    ) -> Result<Option<Value>, Encoding::Error> {
        if let Some(class_hash) = &event.class_hash {
            if let Some(class_hashes) = encoding.get_field(PhantomData).get() {
                if class_hashes.contains(class_hash) {
                    let value = encoding.decode(event)?;
                    return Ok(Some(value));
                }
            }
        }

        Ok(None)
    }
}
