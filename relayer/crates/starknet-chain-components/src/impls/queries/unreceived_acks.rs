use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::queries::unreceived_acks_sequences::UnreceivedAcksSequencesQuerier;
use hermes_chain_components::traits::types::ibc::{
    HasChannelIdType, HasIbcChainTypes, HasPortIdType, HasSequenceType,
};
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::core::host::types::identifiers::{PortId as IbcPortId, Sequence as IbcSequence};
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::contract::call::CanCallContract;
use crate::traits::queries::address::CanQueryContractAddress;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;
use crate::types::channel_id::ChannelId;
use crate::types::messages::ibc::channel::PortId as CairoPortId;
use crate::types::messages::ibc::packet::Sequences;

pub struct QueryStarknetUnreceivedAckSequences;

impl<Chain, Counterparty, Encoding> UnreceivedAcksSequencesQuerier<Chain, Counterparty>
    for QueryStarknetUnreceivedAckSequences
where
    Chain: HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasSequenceType<Counterparty, Sequence = IbcSequence>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasIbcChainTypes<Chain, Sequence = IbcSequence>,
    Encoding: CanEncode<ViaCairo, Product![CairoPortId, ChannelId, Sequences]>
        + CanDecode<ViaCairo, Product![Sequences]>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_unreceived_acknowledgments_sequences(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &IbcPortId,
        packet_ack_sequences: &[IbcSequence],
    ) -> Result<Vec<IbcSequence>, Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let cairo_sequences = packet_ack_sequences
            .iter()
            .map(|sequence| sequence.value())
            .collect::<Vec<_>>();

        let sequences = Sequences {
            sequences: cairo_sequences,
        };

        let calldata = encoding
            .encode(&product![port_id.clone(), channel_id.clone(), sequences])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("unreceived_ack_sequences"),
                &calldata,
            )
            .await?;

        let product![packet_sequences,] = encoding.decode(&output).map_err(Chain::raise_error)?;

        let unreceived_ack_sequences = packet_sequences
            .sequences
            .into_iter()
            .map(IbcSequence::from)
            .collect::<Vec<_>>();

        Ok(unreceived_ack_sequences)
    }
}
