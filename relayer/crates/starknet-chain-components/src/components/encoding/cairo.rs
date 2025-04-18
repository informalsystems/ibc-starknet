#[cgp::re_export_imports]
mod preset {
    use core::time::Duration;

    use cgp::core::component::{UseContext, UseDelegate};
    use cgp::prelude::*;
    use hermes_cairo_encoding_components::components::encode_mut::CairoEncodeMutComponents;
    use hermes_cairo_encoding_components::components::encoding::*;
    use hermes_cairo_encoding_components::impls::encode_mut::cons::EncoderCons;
    use hermes_cairo_encoding_components::impls::encode_mut::display::EncodeDisplay;
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
    use ibc::clients::tendermint::types::{Header as TendermintLcHeader, TrustThreshold};
    use ibc::core::commitment_types::specs::ProofSpecs;
    use ibc::core::host::types::identifiers::ChainId;
    use ibc_proto::google::protobuf::Timestamp as ProtoTimestamp;
    use ibc_proto::ics23::{InnerSpec, LeafOp, ProofSpec};
    use starknet::core::types::{ByteArray, Felt, U256};
    use tendermint::block::header::Version as HeaderVersion;
    use tendermint::block::parts::Header as PartSetHeader;
    use tendermint::block::signed_header::SignedHeader;
    use tendermint::block::Header as TmHeader;
    use tendermint::hash::Hash as TmHash;
    use tendermint::validator::ProposerPriority;
    use tendermint::{account, block, validator, vote, AppHash, PublicKey, Signature};

    use crate::impls::types::address::StarknetAddress;
    use crate::types::channel_id::{
        ChannelEnd, ChannelId, EncodeChannelEnd, RawChannelCounterparty, RawChannelEnd,
        RawChannelState,
    };
    use crate::types::client_id::{ClientId, EncodeClientId};
    use crate::types::connection_id::{
        ConnectionCounterparty, ConnectionEnd, ConnectionId, ConnectionState,
        EncodeConnectionCounterparty, EncodeConnectionEnd, EncodeConnectionState, EncodeDuration,
    };
    use crate::types::cosmos::client_state::{
        ClientStatus, CometClientState, EncodeInnerSpec, EncodeLeafOp, EncodeProofSpec,
        EncodeProofSpecs, EncodeTrustThreshold,
    };
    use crate::types::cosmos::consensus_state::CometConsensusState;
    use crate::types::cosmos::height::{EncodeHeight, Height};
    use crate::types::cosmos::timestamp::{EncodeTimestamp, Timestamp};
    use crate::types::cosmos::update::{
        EncodeAccountId, EncodeAppHash, EncodeBlockId, EncodeCommit, EncodeCommitBlockIdFlag,
        EncodeCommitSig, EncodeHeaderVersion, EncodePartSetHeader, EncodeProposerPriority,
        EncodeProtoTimestamp, EncodePublicKey, EncodeSignature, EncodeSignedHeader,
        EncodeTendermintLcHeader, EncodeTmHash, EncodeTmHeader, EncodeValidator,
        EncodeValidatorSet, EncodeVotePower,
    };
    use crate::types::message_responses::create_client::CreateClientResponse;
    use crate::types::messages::erc20::deploy::DeployErc20TokenMessage;
    use crate::types::messages::erc20::transfer::TransferErc20TokenMessage;
    use crate::types::messages::ibc::channel::{
        AppVersion, ChannelOrdering, EncodeChannelOrdering, MsgChanOpenAck, MsgChanOpenConfirm,
        MsgChanOpenInit, MsgChanOpenTry, PortId,
    };
    use crate::types::messages::ibc::connection::{
        BasePrefix, ConnectionVersion, EncodeBasePrefix, EncodeConnectionVersion, MsgConnOpenAck,
        MsgConnOpenConfirm, MsgConnOpenInit, MsgConnOpenTry,
    };
    use crate::types::messages::ibc::denom::{Denom, PrefixedDenom, TracePrefix};
    use crate::types::messages::ibc::ibc_transfer::{MsgTransfer, Participant, TransferPacketData};
    use crate::types::messages::ibc::packet::{
        AckStatus, Acknowledgement, EncodePacket, EncodeSequence, MsgAckPacket, MsgRecvPacket,
        MsgTimeoutPacket, Packet, Sequence, StateProof,
    };
    use crate::types::register::{MsgRegisterApp, MsgRegisterClient};

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
            (ViaCairo, Height): EncodeHeight,
            (ViaCairo, Timestamp): EncodeTimestamp,
            (ViaCairo, Packet): EncodePacket,
            (ViaCairo, Sequence): EncodeSequence,
            (ViaCairo, ClientId): EncodeClientId,
            (ViaCairo, Duration): EncodeDuration,
            (ViaCairo, ConnectionCounterparty): EncodeConnectionCounterparty,
            (ViaCairo, ConnectionState): EncodeConnectionState,
            (ViaCairo, ConnectionEnd): EncodeConnectionEnd,
            (ViaCairo, TendermintLcHeader): EncodeTendermintLcHeader,
            (ViaCairo, TrustThreshold): EncodeTrustThreshold,
            (ViaCairo, SignedHeader): EncodeSignedHeader,
            (ViaCairo, HeaderVersion): EncodeHeaderVersion,
            (ViaCairo, AppHash): EncodeAppHash,
            (ViaCairo, TmHeader): EncodeTmHeader,
            (ViaCairo, block::Commit): EncodeCommit,
            (ViaCairo, TmHash): EncodeTmHash,
            (ViaCairo, block::Id): EncodeBlockId,
            (ViaCairo, PartSetHeader): EncodePartSetHeader,
            (ViaCairo, Signature): EncodeSignature,
            (ViaCairo, ProtoTimestamp): EncodeProtoTimestamp,
            (ViaCairo, block::BlockIdFlag): EncodeCommitBlockIdFlag,
            (ViaCairo, block::CommitSig): EncodeCommitSig,
            (ViaCairo, validator::Set): EncodeValidatorSet,
            (ViaCairo, vote::Power): EncodeVotePower,
            (ViaCairo, validator::Info): EncodeValidator,
            (ViaCairo, account::Id): EncodeAccountId,
            (ViaCairo, ProposerPriority): EncodeProposerPriority,
            (ViaCairo, PublicKey): EncodePublicKey,
            (ViaCairo, BasePrefix): EncodeBasePrefix,
            (ViaCairo, ConnectionVersion): EncodeConnectionVersion,
            (ViaCairo, ChannelOrdering): EncodeChannelOrdering,
            (ViaCairo, ChannelEnd): EncodeChannelEnd,
            (ViaCairo, InnerSpec): EncodeInnerSpec,
            (ViaCairo, LeafOp): EncodeLeafOp,
            (ViaCairo, ProofSpec): EncodeProofSpec,
            (ViaCairo, ProofSpecs): EncodeProofSpecs,
            [
                (ViaCairo, Vec<Sequence>),
                (ViaCairo, Vec<TracePrefix>),
                (ViaCairo, Vec<validator::Info>),
                (ViaCairo, Vec<block::CommitSig>),
                (ViaCairo, Vec<ProofSpec>),
            ]: EncodeList,
            [
                (ViaCairo, ChainId),
                (ViaCairo, ConnectionId),
                (ViaCairo, ChannelId),
                (ViaCairo, PortId),
                (ViaCairo, AppVersion),
            ]: EncodeDisplay,
            [
                (ViaCairo, MsgRegisterClient),
                (ViaCairo, MsgRegisterApp),
                (ViaCairo, TracePrefix),
                (ViaCairo, PrefixedDenom),
                (ViaCairo, CometClientState),
                (ViaCairo, CometConsensusState),
                (ViaCairo, MsgConnOpenInit),
                (ViaCairo, MsgConnOpenTry),
                (ViaCairo, MsgConnOpenAck),
                (ViaCairo, MsgConnOpenConfirm),
                (ViaCairo, MsgChanOpenInit),
                (ViaCairo, MsgChanOpenTry),
                (ViaCairo, MsgChanOpenAck),
                (ViaCairo, MsgChanOpenConfirm),
                (ViaCairo, MsgRecvPacket),
                (ViaCairo, MsgTimeoutPacket),
                (ViaCairo, MsgAckPacket),
                (ViaCairo, Acknowledgement),
                (ViaCairo, MsgTransfer),
                (ViaCairo, TransferPacketData),
                (ViaCairo, DeployErc20TokenMessage),
                (ViaCairo, TransferErc20TokenMessage),
                (ViaCairo, CreateClientResponse),
                (ViaCairo, StateProof),
                (ViaCairo, StarknetAddress),
                (ViaCairo, RawChannelCounterparty),
                (ViaCairo, RawChannelEnd),
            ]: EncodeFields,
            [
                (ViaCairo, Denom),
                (ViaCairo, AckStatus),
                (ViaCairo, Participant),
                (ViaCairo, ClientStatus),
                (ViaCairo, RawChannelState),
            ]:
                EncodeEnumFields,
        }
    }
}
