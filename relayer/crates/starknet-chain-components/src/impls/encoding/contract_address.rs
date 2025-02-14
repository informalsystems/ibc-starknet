use core::marker::PhantomData;
use std::collections::HashSet;

use cgp::prelude::HasField;
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};

use crate::impls::types::address::StarknetAddress;
use crate::types::event::StarknetEvent;

pub struct DecodeOptionalByContractAddress<Tag>(pub PhantomData<Tag>);

impl<Encoding, Strategy, Value, Tag> Decoder<Encoding, Strategy, Option<Value>>
    for DecodeOptionalByContractAddress<Tag>
where
    Encoding: CanDecode<Strategy, Value, Encoded = StarknetEvent>
        + HasField<Tag, Value = HashSet<StarknetAddress>>,
{
    fn decode(
        encoding: &Encoding,
        event: &StarknetEvent,
    ) -> Result<Option<Value>, Encoding::Error> {
        let contract_addresses = encoding.get_field(PhantomData);

        if contract_addresses.contains(&event.contract_address) {
            let value = encoding.decode(event)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}
