use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::HList;
use hermes_encoding_components::traits::decoder::CanDecode;
use starknet::core::types::{Felt, U256};

use crate::traits::event::{CanDecodeStarknetEvent, StarknetEventDecoder};
use crate::traits::types::method::HasSelectorType;

pub enum Erc20Event {
    Transfer(TransferEvent),
    Approval(ApprovalEvent),
}

pub struct TransferEvent {
    pub from: Felt,
    pub to: Felt,
    pub value: U256,
}

pub struct ApprovalEvent {
    pub owner: Felt,
    pub spender: Felt,
    pub value: U256,
}

pub struct DecodeErc20Events;

impl<Encoding> StarknetEventDecoder<Encoding, Erc20Event> for DecodeErc20Events
where
    Encoding: HasSelectorType<Selector = Felt>
        + CanDecodeStarknetEvent<TransferEvent>
        + CanDecodeStarknetEvent<ApprovalEvent>,
{
    fn decode_event(
        encoding: &Encoding,
        selector: &Encoding::Selector,
        keys: &Encoding::Encoded,
        values: &Encoding::Encoded,
    ) -> Result<Erc20Event, Encoding::Error> {
        todo!()
    }
}

impl<Encoding> StarknetEventDecoder<Encoding, TransferEvent> for DecodeErc20Events
where
    Encoding: HasSelectorType + CanDecode<ViaCairo, HList![Felt, Felt]> + CanDecode<ViaCairo, U256>,
{
    fn decode_event(
        encoding: &Encoding,
        _selector: &Encoding::Selector,
        keys: &Encoding::Encoded,
        values: &Encoding::Encoded,
    ) -> Result<TransferEvent, Encoding::Error> {
        let (from, (to, ())) = encoding.decode(keys)?;
        let value = encoding.decode(values)?;

        Ok(TransferEvent { from, to, value })
    }
}

impl<Encoding> StarknetEventDecoder<Encoding, ApprovalEvent> for DecodeErc20Events
where
    Encoding: HasSelectorType + CanDecode<ViaCairo, HList![Felt, Felt]> + CanDecode<ViaCairo, U256>,
{
    fn decode_event(
        encoding: &Encoding,
        _selector: &Encoding::Selector,
        keys: &Encoding::Encoded,
        values: &Encoding::Encoded,
    ) -> Result<ApprovalEvent, Encoding::Error> {
        let (owner, (spender, ())) = encoding.decode(keys)?;
        let value = encoding.decode(values)?;

        Ok(ApprovalEvent {
            owner,
            spender,
            value,
        })
    }
}
