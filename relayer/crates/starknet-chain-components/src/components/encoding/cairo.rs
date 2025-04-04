#[cgp::re_export_imports]
mod preset {
    use core::time::Duration;

    use cgp::core::component::{UseContext, UseDelegate};
    use cgp::prelude::*;
    use hermes_cairo_encoding_components::components::encode_mut::CairoEncodeMutComponents;
    use hermes_cairo_encoding_components::components::encoding::*;
    use hermes_cairo_encoding_components::impls::encode_mut::cons::EncoderCons;
    use hermes_cairo_encoding_components::impls::encode_mut::enum_fields::EncodeEnumFields;
    use hermes_cairo_encoding_components::impls::encode_mut::option::EncodeOption;
    use hermes_cairo_encoding_components::impls::encode_mut::pair::EncoderPair;
    use hermes_cairo_encoding_components::impls::encode_mut::reference::EncodeDeref;
    use hermes_cairo_encoding_components::impls::encode_mut::vec::EncodeList;
    use hermes_cairo_encoding_components::strategy::ViaCairo;
    use hermes_encoding_components::impls::fields::EncodeFields;
    use hermes_encoding_components::impls::tagged::EncodeTaggedField;
    use hermes_encoding_components::traits::decode::DecoderComponent;
    use hermes_encoding_components::traits::decode_mut::{
        DecodeBufferPeekerComponent, MutDecoderComponent,
    };
    use hermes_encoding_components::traits::encode::EncoderComponent;
    use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
    use hermes_encoding_components::traits::types::decode_buffer::{
        DecodeBufferBuilderComponent, DecodeBufferTypeComponent,
    };
    use hermes_encoding_components::traits::types::encode_buffer::{
        EncodeBufferFinalizerComponent, EncodeBufferTypeComponent,
    };
    use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
    use ibc::core::host::types::identifiers::ChainId;
    use starknet::core::types::{Felt, U256};

    use crate::impls::types::address::{EncodeStarknetAddress, StarknetAddress};
    use crate::types::channel_id::{
        ChannelCounterparty, ChannelEnd, ChannelId, ChannelState, EncodeChannelCounterparty,
        EncodeChannelEnd, EncodeChannelId, EncodeChannelState,
    };
    use crate::types::client_id::{ClientId, EncodeClientId};
    use crate::types::connection_id::{
        ConnectionCounterparty, ConnectionEnd, ConnectionId, ConnectionState,
        EncodeConnectionCounterparty, EncodeConnectionEnd, EncodeConnectionId,
        EncodeConnectionState, EncodeDuration,
    };
    use crate::types::cosmos::client_state::{
        ClientStatus, CometClientState, EncodeChainId, EncodeClientStatus, EncodeCometClientState,
    };
    use crate::types::cosmos::consensus_state::{CometConsensusState, EncodeCometConsensusState};
    use crate::types::cosmos::height::{EncodeHeight, Height};
    use crate::types::cosmos::timestamp::{EncodeTimestamp, Timestamp};
    use crate::types::cosmos::update::{CometUpdateHeader, EncodeCometUpdateHeader};
    use crate::types::message_responses::create_client::{
        CreateClientResponse, DecodeCreateClientResponse,
    };
    use crate::types::messages::erc20::deploy::{
        DeployErc20TokenMessage, EncodeDeployErc20TokenMessage,
    };
    use crate::types::messages::erc20::transfer::{
        EncodeTransferErc20TokenMessage, TransferErc20TokenMessage,
    };
    use crate::types::messages::ibc::channel::{
        AppVersion, ChannelOrdering, EncodeAppVersion, EncodeChannelOrdering, EncodePortId,
        MsgChanOpenAck, MsgChanOpenConfirm, MsgChanOpenInit, MsgChanOpenTry, PortId,
    };
    use crate::types::messages::ibc::connection::{
        BasePrefix, ConnectionVersion, EncodeBasePrefix, EncodeConnectionVersion, MsgConnOpenAck,
        MsgConnOpenConfirm, MsgConnOpenInit, MsgConnOpenTry,
    };
    use crate::types::messages::ibc::denom::{Denom, PrefixedDenom, TracePrefix};
    use crate::types::messages::ibc::ibc_transfer::{
        EncodeMsgTransfer, EncodeParticipant, EncodeTransferPacketData, MsgTransfer, Participant,
        TransferPacketData,
    };
    use crate::types::messages::ibc::packet::{
        AckStatus, Acknowledgement, EncodeAckStatus, EncodeAcknowledgement, EncodeMsgAckPacket,
        EncodeMsgRecvPacket, EncodeMsgTimeoutPacket, EncodePacket, EncodeSequence,
        EncodeStateProof, MsgAckPacket, MsgRecvPacket, MsgTimeoutPacket, Packet, Sequence,
        StateProof,
    };
    use crate::types::register::{
        EncodeRegisterApp, EncodeRegisterClient, MsgRegisterApp, MsgRegisterClient,
    };

    cgp_preset! {
        StarknetCairoEncodingComponents {
            [
                EncodedTypeComponent,
                EncodeBufferTypeComponent,
                EncodeBufferFinalizerComponent,
                DecodeBufferTypeComponent,
                DecodeBufferBuilderComponent,
                DecodeBufferPeekerComponent,
                EncoderComponent,
                DecoderComponent,
            ]: CairoEncodingComponents::Provider,
            [
                MutEncoderComponent,
                MutDecoderComponent,
            ]:
                UseDelegate<StarknetEncodeMutComponents>
        }
    }

    pub struct StarknetEncodeMutComponents;

    CairoEncodeMutComponents::with_components! {
        | Components | {
            delegate_components! {
                StarknetEncodeMutComponents {
                    Components: UseDelegate<CairoEncodeMutComponents::Provider>,
                }
            }
        }
    }

    delegate_components! {
        StarknetEncodeMutComponents {
            <'a, V> (ViaCairo, &'a V): EncodeDeref,
            <V> (ViaCairo, Option<V>): EncodeOption<V>,
            <A, B> (ViaCairo, (A, B)): EncoderPair<UseContext, UseContext>,
            <A, B> (ViaCairo, Cons<A, B>): EncoderCons<UseContext, UseContext>,
            <Tag, Value> (ViaCairo, Field<Tag, Value>): EncodeTaggedField,
            (ViaCairo, TransferErc20TokenMessage): EncodeTransferErc20TokenMessage,
            (ViaCairo, DeployErc20TokenMessage): EncodeDeployErc20TokenMessage,
            (ViaCairo, Vec<TracePrefix>): EncodeList,
            (ViaCairo, Participant): EncodeParticipant,
            (ViaCairo, TransferPacketData): EncodeTransferPacketData,
            (ViaCairo, MsgTransfer): EncodeMsgTransfer,
            (ViaCairo, Height): EncodeHeight,
            (ViaCairo, Timestamp): EncodeTimestamp,
            (ViaCairo, StarknetAddress): EncodeStarknetAddress,
            (ViaCairo, Packet): EncodePacket,
            (ViaCairo, StateProof): EncodeStateProof,
            (ViaCairo, MsgRecvPacket): EncodeMsgRecvPacket,
            (ViaCairo, Acknowledgement): EncodeAcknowledgement,
            (ViaCairo, MsgAckPacket): EncodeMsgAckPacket,
            (ViaCairo, AckStatus): EncodeAckStatus,
            (ViaCairo, Sequence): EncodeSequence,
            (ViaCairo, Vec<Sequence>): EncodeList,
            (ViaCairo, MsgTimeoutPacket): EncodeMsgTimeoutPacket,
            (ViaCairo, ClientStatus): EncodeClientStatus,
            (ViaCairo, CometClientState): EncodeCometClientState,
            (ViaCairo, CometConsensusState): EncodeCometConsensusState,
            (ViaCairo, ClientId): EncodeClientId,
            (ViaCairo, ChainId): EncodeChainId,
            (ViaCairo, ConnectionId): EncodeConnectionId,
            (ViaCairo, Duration): EncodeDuration,
            (ViaCairo, ConnectionCounterparty): EncodeConnectionCounterparty,
            (ViaCairo, ConnectionState): EncodeConnectionState,
            (ViaCairo, ConnectionEnd): EncodeConnectionEnd,
            (ViaCairo, ChannelId): EncodeChannelId,
            (ViaCairo, ChannelState): EncodeChannelState,
            (ViaCairo, ChannelCounterparty): EncodeChannelCounterparty,
            (ViaCairo, ChannelEnd): EncodeChannelEnd,
            (ViaCairo, CometUpdateHeader): EncodeCometUpdateHeader,
            (ViaCairo, CreateClientResponse): DecodeCreateClientResponse,
            (ViaCairo, MsgRegisterClient): EncodeRegisterClient,
            (ViaCairo, MsgRegisterApp): EncodeRegisterApp,
            (ViaCairo, BasePrefix): EncodeBasePrefix,
            (ViaCairo, ConnectionVersion): EncodeConnectionVersion,
            (ViaCairo, PortId): EncodePortId,
            (ViaCairo, AppVersion): EncodeAppVersion,
            (ViaCairo, ChannelOrdering): EncodeChannelOrdering,
            [
                (ViaCairo, TracePrefix),
                (ViaCairo, PrefixedDenom),
                (ViaCairo, MsgConnOpenInit),
                (ViaCairo, MsgConnOpenTry),
                (ViaCairo, MsgConnOpenAck),
                (ViaCairo, MsgConnOpenConfirm),
                (ViaCairo, MsgChanOpenInit),
                (ViaCairo, MsgChanOpenTry),
                (ViaCairo, MsgChanOpenAck),
                (ViaCairo, MsgChanOpenConfirm),
            ]: EncodeFields,
            [
                (ViaCairo, Denom),
            ]:
                EncodeEnumFields,
        }
    }
}
