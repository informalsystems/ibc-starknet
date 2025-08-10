#[cgp::re_export_imports]
mod preset {
    use core::time::Duration;

    use cgp::core::component::{UseContext, UseDelegate};
    use hermes_cairo_encoding_components::components::encode_mut::CairoEncodeMutComponents;
    use hermes_cairo_encoding_components::components::encoding::*;
    use hermes_cairo_encoding_components::impls::{
        EncodeDeref, EncodeDisplay, EncodeEnumFields, EncodeList, EncodeOption, EncoderCons,
        EncoderPair,
    };
    use hermes_cairo_encoding_components::strategy::ViaCairo;
    use hermes_core::encoding_components::impls::{EncodeFields, EncodeTaggedField};
    use hermes_core::encoding_components::traits::{
        DecodeBufferBuilderComponent, DecodeBufferPeekerComponent, DecodeBufferTypeComponent,
        DecoderComponent, EncodeBufferFinalizerComponent, EncodeBufferTypeComponent,
        EncodedTypeComponent, EncoderComponent, MutDecoderComponent, MutEncoderComponent,
    };
    use hermes_prelude::*;
    use ibc::clients::tendermint::types::{Header as TendermintLcHeader, TrustThreshold};
    use ibc::core::channel::types::channel::Order as ChannelOrdering;
    use ibc::core::channel::types::Version as AppVersion;
    use ibc::core::commitment_types::specs::ProofSpecs;
    use ibc::core::host::types::identifiers::{ChainId, ChannelId, ConnectionId, PortId};
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

    use crate::impls::StarknetAddress;
    use crate::types::{
        AckStatus, Acknowledgement, BasePrefix, CairoStarknetClientState,
        CairoStarknetConsensusState, ChannelEnd, ClientId, ClientMessage, ClientStatus,
        CometClientState, CometConsensusState, ConnectionCounterparty, ConnectionEnd,
        ConnectionState, ConnectionVersion, CreateClientResponse, Denom, DeployErc20TokenMessage,
        EncodeAccountId, EncodeAppHash, EncodeBasePrefix, EncodeBlockId, EncodeChannelEnd,
        EncodeChannelOrdering, EncodeClientId, EncodeCommit, EncodeCommitBlockIdFlag,
        EncodeCommitSig, EncodeConnectionCounterparty, EncodeConnectionEnd, EncodeConnectionState,
        EncodeConnectionVersion, EncodeDuration, EncodeHeaderVersion, EncodeInnerSpec,
        EncodeLeafOp, EncodePacket, EncodePartSetHeader, EncodeProofSpec, EncodeProofSpecs,
        EncodeProposerPriority, EncodeProtoTimestamp, EncodePublicKey, EncodeSequence,
        EncodeSignature, EncodeSignedHeader, EncodeTendermintLcHeader, EncodeTimestamp,
        EncodeTmHash, EncodeTmHeader, EncodeTrustThreshold, EncodeValidator, EncodeValidatorSet,
        EncodeVotePower, Height, MsgAckPacket, MsgChanOpenAck, MsgChanOpenConfirm, MsgChanOpenInit,
        MsgChanOpenTry, MsgConnOpenAck, MsgConnOpenConfirm, MsgConnOpenInit, MsgConnOpenTry,
        MsgRecvPacket, MsgRegisterApp, MsgRegisterClient, MsgTimeoutPacket, MsgTransfer, Packet,
        Participant, PrefixedDenom, RawChannelCounterparty, RawChannelEnd, RawChannelState,
        Sequence, StateProof, Timestamp, TracePrefix, TransferErc20TokenMessage,
        TransferPacketData,
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
                (ViaCairo, Height),
                (ViaCairo, CairoStarknetClientState),
                (ViaCairo, CairoStarknetConsensusState),
            ]: EncodeFields,
            [
                (ViaCairo, Denom),
                (ViaCairo, AckStatus),
                (ViaCairo, Participant),
                (ViaCairo, ClientStatus),
                (ViaCairo, RawChannelState),
                (ViaCairo, ClientMessage),
            ]:
                EncodeEnumFields,
        }
    }
}
