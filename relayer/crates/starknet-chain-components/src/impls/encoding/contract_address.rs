use core::marker::PhantomData;
use std::collections::HashSet;
use std::sync::OnceLock;

use hermes_core::encoding_components::traits::{CanDecode, Decoder, DecoderComponent};
use hermes_prelude::*;

use crate::impls::StarknetAddress;
use crate::types::StarknetEvent;

pub struct DecodeOptionalByContractAddress<Tag>(pub PhantomData<Tag>);

#[cgp_provider(DecoderComponent)]
impl<Encoding, Strategy, Value, Tag> Decoder<Encoding, Strategy, Option<Value>>
    for DecodeOptionalByContractAddress<Tag>
where
    Encoding: CanDecode<Strategy, Value, Encoded = StarknetEvent>
        + HasField<Tag, Value = OnceLock<HashSet<StarknetAddress>>>,
{
    fn decode(
        encoding: &Encoding,
        event: &StarknetEvent,
    ) -> Result<Option<Value>, Encoding::Error> {
        if let Some(contract_addresses) = encoding.get_field(PhantomData).get() {
            if contract_addresses.contains(&event.contract_address) {
                let value = encoding.decode(event)?;
                return Ok(Some(value));
            }
        }

        Ok(None)
    }
}
