use cgp::core::component::{UseContext, UseDelegate};
use cgp::prelude::*;
use hermes_cairo_encoding_components::components::encode_mut::*;
pub use hermes_cairo_encoding_components::components::encoding::*;
use hermes_cairo_encoding_components::impls::encode_mut::cons::EncoderCons;
use hermes_cairo_encoding_components::impls::encode_mut::option::EncodeOption;
use hermes_cairo_encoding_components::impls::encode_mut::pair::EncoderPair;
use hermes_cairo_encoding_components::impls::encode_mut::reference::EncodeDeref;
use hermes_cairo_encoding_components::impls::encode_mut::vec::EncodeList;
use hermes_cairo_encoding_components::strategy::ViaCairo;
pub use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
pub use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use starknet::core::types::{Felt, U256};

use crate::types::channel_id::{ChannelId, EncodeChannelId};
use crate::types::client_id::{ClientId, EncodeClientId};
use crate::types::connection_id::{ConnectionId, EncodeConnectionId};
use crate::types::cosmos::client_state::{
    ClientStatus, CometClientState, EncodeClientStatus, EncodeCometClientState,
};
use crate::types::cosmos::consensus_state::{CometConsensusState, EncodeCometConsensusState};
use crate::types::cosmos::height::{EncodeHeight, Height};
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
    AppVersion, ChannelOrdering, EncodeAppVersion, EncodeChannelOrdering, EncodeMsgChanOpenAck,
    EncodeMsgChanOpenConfirm, EncodeMsgChanOpenInit, EncodeMsgChanOpenTry, EncodePortId,
    MsgChanOpenAck, MsgChanOpenConfirm, MsgChanOpenInit, MsgChanOpenTry, PortId,
};
use crate::types::messages::ibc::connection::{
    BasePrefix, ConnectionVersion, EncodeBasePrefix, EncodeConnectionVersion, EncodeMsgConnOpenAck,
    EncodeMsgConnOpenConfirm, EncodeMsgConnOpenInit, EncodeMsgConnOpenTry, MsgConnOpenAck,
    MsgConnOpenConfirm, MsgConnOpenInit, MsgConnOpenTry,
};
use crate::types::messages::ibc::denom::{
    Denom, EncodeDenom, EncodePrefixedDenom, EncodeTracePrefix, PrefixedDenom, TracePrefix,
};
use crate::types::messages::ibc::ibc_transfer::{
    EncodeIbcTransferMessage, EncodeParticipant, IbcTransferMessage, Participant,
};
use crate::types::messages::ibc::packet::{
    EncodeMsgRecvPacket, EncodePacket, EncodeStateProof, MsgRecvPacket, Packet, StateProof,
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
        ]: CairoEncodingComponents,
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]:
            UseDelegate<StarknetEncodeMutComponents>
    }
}

pub struct StarknetEncodeMutComponents;

with_cairo_encode_mut_components! {
    | Components | {
        delegate_components! {
            StarknetEncodeMutComponents {
                Components: UseDelegate<CairoEncodeMutComponents>,
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
        (ViaCairo, TransferErc20TokenMessage): EncodeTransferErc20TokenMessage,
        (ViaCairo, DeployErc20TokenMessage): EncodeDeployErc20TokenMessage,
        (ViaCairo, Denom): EncodeDenom,
        (ViaCairo, TracePrefix): EncodeTracePrefix,
        (ViaCairo, Vec<TracePrefix>): EncodeList,
        (ViaCairo, PrefixedDenom): EncodePrefixedDenom,
        (ViaCairo, Participant): EncodeParticipant,
        (ViaCairo, IbcTransferMessage): EncodeIbcTransferMessage,
        (ViaCairo, Height): EncodeHeight,
        (ViaCairo, Packet): EncodePacket,
        (ViaCairo, StateProof): EncodeStateProof,
        (ViaCairo, MsgRecvPacket): EncodeMsgRecvPacket,
        (ViaCairo, ClientStatus): EncodeClientStatus,
        (ViaCairo, CometClientState): EncodeCometClientState,
        (ViaCairo, CometConsensusState): EncodeCometConsensusState,
        (ViaCairo, ClientId): EncodeClientId,
        (ViaCairo, ConnectionId): EncodeConnectionId,
        (ViaCairo, ChannelId): EncodeChannelId,
        (ViaCairo, CometUpdateHeader): EncodeCometUpdateHeader,
        (ViaCairo, CreateClientResponse): DecodeCreateClientResponse,
        (ViaCairo, MsgRegisterClient): EncodeRegisterClient,
        (ViaCairo, MsgRegisterApp): EncodeRegisterApp,
        (ViaCairo, BasePrefix): EncodeBasePrefix,
        (ViaCairo, ConnectionVersion): EncodeConnectionVersion,
        (ViaCairo, MsgConnOpenInit): EncodeMsgConnOpenInit,
        (ViaCairo, MsgConnOpenTry): EncodeMsgConnOpenTry,
        (ViaCairo, MsgConnOpenAck): EncodeMsgConnOpenAck,
        (ViaCairo, MsgConnOpenConfirm): EncodeMsgConnOpenConfirm,
        (ViaCairo, PortId): EncodePortId,
        (ViaCairo, AppVersion): EncodeAppVersion,
        (ViaCairo, ChannelOrdering): EncodeChannelOrdering,
        (ViaCairo, MsgChanOpenInit): EncodeMsgChanOpenInit,
        (ViaCairo, MsgChanOpenTry): EncodeMsgChanOpenTry,
        (ViaCairo, MsgChanOpenAck): EncodeMsgChanOpenAck,
        (ViaCairo, MsgChanOpenConfirm): EncodeMsgChanOpenConfirm,
    }
}
