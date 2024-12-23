use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_components::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilder, ChannelOpenConfirmMessageBuilder, ChannelOpenInitMessageBuilder,
    ChannelOpenTryMessageBuilder,
};
use hermes_chain_components::traits::types::channel::{
    HasChannelEndType, HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType,
    HasChannelOpenTryPayloadType, HasInitChannelOptionsType,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_chain_components::traits::types::proof::HasCommitmentProofType;
use hermes_chain_components::types::payloads::channel::ChannelOpenTryPayload;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use ibc::core::channel::types::channel::{ChannelEnd, Order as IbcOrder};
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChannelId, PortId as IbcPortId};
use starknet::accounts::Call;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::traits::queries::address::CanQueryContractAddress;
use crate::types::channel_id::ChannelId as StarknetChannelId;
use crate::types::connection_id::ConnectionId as StarknetConnectionId;
use crate::types::cosmos::height::Height as CairoHeight;
use crate::types::messages::ibc::channel::{
    AppVersion, ChannelOrdering, MsgChanOpenInit, MsgChanOpenTry, PortId as StarknetPortId,
};
use crate::types::messages::ibc::packet::StateProof;
pub struct BuildStarknetChannelHandshakeMessages;

impl<Chain, Counterparty, Encoding> ChannelOpenInitMessageBuilder<Chain, Counterparty>
    for BuildStarknetChannelHandshakeMessages
where
    Chain: HasMessageType<Message = Call>
        + HasAddressType<Address = Felt>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasInitChannelOptionsType<Counterparty, InitChannelOptions = CosmosInitChannelOptions>
        + CanRaiseError<&'static str>
        + CanRaiseError<Encoding::Error>,
    Counterparty: HasPortIdType<Chain, PortId = IbcPortId>,
    Encoding: CanEncode<ViaCairo, MsgChanOpenInit> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_channel_open_init_message(
        chain: &Chain,
        port_id: &IbcPortId,
        counterparty_port_id: &IbcPortId,
        init_channel_options: &CosmosInitChannelOptions,
    ) -> Result<Chain::Message, Chain::Error> {
        let port_id_on_a = StarknetPortId {
            port_id: port_id.to_string(),
        };

        let port_id_on_b = StarknetPortId {
            port_id: counterparty_port_id.to_string(),
        };

        if init_channel_options.connection_hops.len() != 1 {
            return Err(Chain::raise_error(
                "Starknet only supports a single connection hop",
            ));
        }

        let conn_id_on_a = StarknetConnectionId {
            connection_id: init_channel_options.connection_hops[0].to_string(),
        };

        let version_proposal = AppVersion {
            version: init_channel_options.channel_version.to_string(),
        };

        let ordering = match init_channel_options.ordering {
            IbcOrder::None => {
                return Err(Chain::raise_error("Starknet does not support no ordering"))
            }
            IbcOrder::Ordered => ChannelOrdering::Ordered,
            IbcOrder::Unordered => ChannelOrdering::Unordered,
        };

        let chan_open_init_msg = MsgChanOpenInit {
            port_id_on_a,
            conn_id_on_a,
            port_id_on_b,
            version_proposal,
            ordering,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&chan_open_init_msg)
            .map_err(Chain::raise_error)?;

        let message = Call {
            to: ibc_core_address,
            selector: selector!("chan_open_init"),
            calldata,
        };

        Ok(message)
    }
}

impl<Chain, Counterparty, Encoding> ChannelOpenTryMessageBuilder<Chain, Counterparty>
    for BuildStarknetChannelHandshakeMessages
where
    Chain: HasMessageType<Message = Call>
        + HasAddressType<Address = Felt>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + CanRaiseError<&'static str>
        + CanRaiseError<Encoding::Error>,
    Counterparty: HasChannelIdType<Chain, ChannelId = ChannelId>
        + HasHeightType<Height = Height>
        + HasPortIdType<Chain, PortId = IbcPortId>
        + HasChannelEndType<Chain, ChannelEnd = ChannelEnd>
        + HasCommitmentProofType
        + HasChannelOpenTryPayloadType<
            Chain,
            ChannelOpenTryPayload = ChannelOpenTryPayload<Counterparty, Chain>,
        >,
    Encoding: CanEncode<ViaCairo, MsgChanOpenTry> + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn build_channel_open_try_message(
        chain: &Chain,
        port_id: &IbcPortId,
        counterparty_port_id: &IbcPortId,
        counterparty_channel_id: &ChannelId,
        counterparty_payload: ChannelOpenTryPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let port_id_on_b = StarknetPortId {
            port_id: port_id.to_string(),
        };

        if counterparty_payload.channel_end.connection_hops.len() != 1 {
            return Err(Chain::raise_error(
                "Starknet only supports a single connection hop",
            ));
        }

        let conn_id_on_b = StarknetConnectionId {
            connection_id: counterparty_payload.channel_end.connection_hops[0].to_string(),
        };

        let port_id_on_a = StarknetPortId {
            port_id: counterparty_port_id.to_string(),
        };

        let chan_id_on_a = StarknetChannelId {
            channel_id: counterparty_channel_id.to_string(),
        };

        let version_on_a = AppVersion {
            version: counterparty_payload.channel_end.version.to_string(),
        };

        let proof_chan_end_on_a = StateProof {
            proof: vec![Felt::ONE],
        };

        let proof_height_on_a = CairoHeight {
            revision_number: counterparty_payload.update_height.revision_number(),
            revision_height: counterparty_payload.update_height.revision_height(),
        };

        let ordering = match counterparty_payload.channel_end.ordering {
            IbcOrder::None => {
                return Err(Chain::raise_error("Starknet does not support no ordering"))
            }
            IbcOrder::Ordered => ChannelOrdering::Ordered,
            IbcOrder::Unordered => ChannelOrdering::Unordered,
        };

        let chan_open_try_msg = MsgChanOpenTry {
            port_id_on_b,
            conn_id_on_b,
            port_id_on_a,
            chan_id_on_a,
            version_on_a,
            proof_chan_end_on_a,
            proof_height_on_a,
            ordering,
        };

        let ibc_core_address = chain.query_contract_address(PhantomData).await?;

        let calldata = chain
            .encoding()
            .encode(&chan_open_try_msg)
            .map_err(Chain::raise_error)?;

        let message = Call {
            to: ibc_core_address,
            selector: selector!("chan_open_try"),
            calldata,
        };

        Ok(message)
    }
}

impl<Chain, Counterparty> ChannelOpenAckMessageBuilder<Chain, Counterparty>
    for BuildStarknetChannelHandshakeMessages
where
    Chain: HasMessageType
        + HasPortIdType<Counterparty>
        + HasChannelIdType<Counterparty>
        + HasErrorType,
    Counterparty:
        HasChannelIdType<Chain> + HasPortIdType<Chain> + HasChannelOpenAckPayloadType<Chain>,
{
    async fn build_channel_open_ack_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
        _counterparty_channel_id: &Counterparty::ChannelId,
        _counterparty_payload: Counterparty::ChannelOpenAckPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}

impl<Chain, Counterparty> ChannelOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildStarknetChannelHandshakeMessages
where
    Chain: HasMessageType
        + HasPortIdType<Counterparty>
        + HasChannelIdType<Counterparty>
        + HasErrorType,
    Counterparty: HasChannelOpenConfirmPayloadType<Chain>,
{
    async fn build_channel_open_confirm_message(
        _chain: &Chain,
        _port_id: &Chain::PortId,
        _channel_id: &Chain::ChannelId,
        _counterparty_payload: Counterparty::ChannelOpenConfirmPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        todo!()
    }
}
