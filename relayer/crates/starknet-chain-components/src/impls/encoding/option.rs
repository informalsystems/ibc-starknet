use core::marker::PhantomData;
use std::collections::HashSet;

use cgp::prelude::HasField;
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use starknet::core::types::Felt;

use crate::types::event::StarknetEvent;

pub struct DecodeOptionalByClassHash<Tag>(pub PhantomData<Tag>);

impl<Encoding, Strategy, Value, Tag> Decoder<Encoding, Strategy, Option<Value>>
    for DecodeOptionalByClassHash<Tag>
where
    Encoding:
        CanDecode<Strategy, Value, Encoded = StarknetEvent> + HasField<Tag, Value = HashSet<Felt>>,
{
    fn decode(
        encoding: &Encoding,
        event: &StarknetEvent,
    ) -> Result<Option<Value>, Encoding::Error> {
        let class_hashes = encoding.get_field(PhantomData);

        if class_hashes.contains(&event.class_hash) {
            let value = encoding.decode(event)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}
