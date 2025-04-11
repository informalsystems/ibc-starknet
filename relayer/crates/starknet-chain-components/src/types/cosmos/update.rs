use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, MutDecoder, MutDecoderComponent,
use ibc::clients::tendermint::types::{
    ConsensusState as TendermintConsensusState, Header as TendermintHeader,
};
use hermes_encoding_components::traits::encode_mut::{
    CanEncodeMut, MutEncoder, MutEncoderComponent,
};
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
use ibc::clients::tendermint::types::Header as TendermintLcHeader;
use ibc::core::client::types::Height as IbcHeight;
use ibc_proto::google::protobuf::Timestamp as ProtoTimestamp;
use tendermint::block::header::Version as HeaderVersion;
use tendermint::block::parts::Header as PartSetHeader;
use tendermint::block::signed_header::SignedHeader;
use tendermint::block::{BlockIdFlag, Commit, CommitSig, Header as TmHeader};
use tendermint::hash::Hash as TmHash;
use tendermint::validator::ProposerPriority;
use tendermint::{account, block, validator, vote, AppHash, PublicKey, Signature};

use crate::types::cosmos::height::Height;

pub struct EncodeTendermintLcHeader;

const ZERO_TIMESTAMP: ProtoTimestamp = ProtoTimestamp {
    seconds: -62135596800,
    nanos: 0,
};

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, TendermintLcHeader>
    for EncodeTendermintLcHeader
where
    Encoding:
        CanEncodeMut<Strategy, Product![SignedHeader, validator::Set, Height, validator::Set]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &TendermintLcHeader,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let cairo_trusted_height = Height {
            revision_number: value.trusted_height.revision_number(),
            revision_height: value.trusted_height.revision_height(),
        };

        encoding.encode_mut(
            &product![
                value.signed_header.clone(),
                value.validator_set.clone(),
                cairo_trusted_height,
                value.trusted_next_validator_set.clone(),
            ],
            buffer,
        )?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, TendermintLcHeader>
    for EncodeTendermintLcHeader
where
    Encoding: CanDecodeMut<Strategy, Product![SignedHeader, validator::Set, Height, validator::Set]>
        + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<TendermintLcHeader, Encoding::Error> {
        let product![
            signed_header,
            validator_set,
            trusted_height,
            trusted_next_validator_set
        ] = encoding.decode_mut(buffer)?;

        let ibc_trusted_height = IbcHeight::new(
            trusted_height.revision_number,
            trusted_height.revision_height,
        )
        .map_err(|_| Encoding::raise_error("invalid trusted height"))?;

        Ok(TendermintLcHeader {
            signed_header,
            validator_set,
            trusted_height: ibc_trusted_height,
            trusted_next_validator_set,
        })
    }
}

pub struct EncodeSignedHeader;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, SignedHeader> for EncodeSignedHeader
where
    Encoding: CanEncodeMut<Strategy, Product![TmHeader, Commit]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &SignedHeader,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(
            &product![value.header.clone(), value.commit.clone()],
            buffer,
        )?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, SignedHeader> for EncodeSignedHeader
where
    Encoding: CanDecodeMut<Strategy, Product![TmHeader, Commit]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<SignedHeader, Encoding::Error> {
        let product![header, commit] = encoding.decode_mut(buffer)?;
        SignedHeader::new(header, commit)
            .map_err(|_| Encoding::raise_error("invalid signed header"))
    }
}

pub struct EncodeHeaderVersion;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, HeaderVersion> for EncodeHeaderVersion
where
    Encoding: CanEncodeMut<Strategy, Product![u64, u64]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &HeaderVersion,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&product![value.block, value.app], buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, HeaderVersion> for EncodeHeaderVersion
where
    Encoding: CanDecodeMut<Strategy, Product![u64, u64]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<HeaderVersion, Encoding::Error> {
        let product![block, app] = encoding.decode_mut(buffer)?;
        Ok(HeaderVersion { block, app })
    }
}

pub struct EncodeAppHash;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, AppHash> for EncodeAppHash
where
    Encoding: CanEncodeMut<Strategy, Vec<u8>>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &AppHash,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.as_bytes().to_vec(), buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, AppHash> for EncodeAppHash
where
    Encoding: CanDecodeMut<Strategy, Vec<u8>> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<AppHash, Encoding::Error> {
        let value = encoding.decode_mut(buffer)?;
        value
            .try_into()
            .map_err(|_| Encoding::raise_error("invalid app hash"))
    }
}

pub struct EncodeTmHeader;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, TmHeader> for EncodeTmHeader
where
    Encoding: CanEncodeMut<
        Strategy,
        Product![
            HeaderVersion,
            String,
            u64,
            ProtoTimestamp,
            block::Id,
            TmHash,
            TmHash,
            TmHash,
            TmHash,
            TmHash,
            AppHash,
            TmHash,
            TmHash,
            account::Id
        ],
    >,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &TmHeader,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(
            &product![
                value.version,
                value.chain_id.clone().into(),
                value.height.value(),
                value.time.into(),
                value.last_block_id.unwrap(),
                value.last_commit_hash.unwrap(),
                value.data_hash.unwrap(),
                value.validators_hash,
                value.next_validators_hash,
                value.consensus_hash,
                value.app_hash.clone(),
                value.last_results_hash.unwrap(),
                value.evidence_hash.unwrap(),
                value.proposer_address
            ],
            buffer,
        )?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, TmHeader> for EncodeTmHeader
where
    Encoding: CanDecodeMut<
            Strategy,
            Product![
                HeaderVersion,
                String,
                u64,
                ProtoTimestamp,
                block::Id,
                TmHash,
                TmHash,
                TmHash,
                TmHash,
                TmHash,
                AppHash,
                TmHash,
                TmHash,
                account::Id,
            ],
        > + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<TmHeader, Encoding::Error> {
        let product![
            version,
            chain_id,
            height,
            proto_timestamp,
            last_block_id,
            last_commit_hash,
            data_hash,
            validators_hash,
            next_validators_hash,
            consensus_hash,
            app_hash,
            last_results_hash,
            evidence_hash,
            proposer_address
        ] = encoding.decode_mut(buffer)?;

        let header = TmHeader {
            version,
            chain_id: chain_id
                .try_into()
                .map_err(|_| Encoding::raise_error("invalid chain id"))?,
            height: height
                .try_into()
                .map_err(|_| Encoding::raise_error("invalid height"))?,
            time: proto_timestamp
                .try_into()
                .map_err(|_| Encoding::raise_error("invalid timestamp"))?,
            last_block_id: Some(last_block_id),
            last_commit_hash: Some(last_commit_hash),
            data_hash: Some(data_hash),
            validators_hash,
            next_validators_hash,
            consensus_hash,
            app_hash,
            last_results_hash: Some(last_results_hash),
            evidence_hash: Some(evidence_hash),
            proposer_address,
        };

        Ok(header)
    }
}

pub struct EncodeCommit;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Commit> for EncodeCommit
where
    Encoding: CanEncodeMut<Strategy, Product![i64, i32, block::Id, Vec<CommitSig>]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Commit,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(
            &product![
                value.height.into(),
                value.round.into(),
                value.block_id,
                value.signatures.clone()
            ],
            buffer,
        )?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Commit> for EncodeCommit
where
    Encoding: CanDecodeMut<Strategy, Product![i64, i32, block::Id, Vec<CommitSig>]>
        + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Commit, Encoding::Error> {
        let product![height, round, block_id, signatures] = encoding.decode_mut(buffer)?;
        Ok(Commit {
            height: height
                .try_into()
                .map_err(|_| Encoding::raise_error("invalid height"))?,
            round: round
                .try_into()
                .map_err(|_| Encoding::raise_error("invalid round"))?,
            block_id,
            signatures,
        })
    }
}

pub struct EncodeTmHash;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, TmHash> for EncodeTmHash
where
    Encoding: CanEncodeMut<Strategy, Vec<u8>>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &TmHash,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.as_bytes().to_vec(), buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, TmHash> for EncodeTmHash
where
    Encoding: CanDecodeMut<Strategy, Vec<u8>> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<TmHash, Encoding::Error> {
        let value = encoding.decode_mut(buffer)?;
        value
            .try_into()
            .map_err(|_| Encoding::raise_error("invalid hash"))
    }
}

pub struct EncodeBlockId;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, block::Id> for EncodeBlockId
where
    Encoding: CanEncodeMut<Strategy, Product![TmHash, PartSetHeader]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &block::Id,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&product![value.hash, value.part_set_header], buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, block::Id> for EncodeBlockId
where
    Encoding:
        CanDecodeMut<Strategy, Product![TmHash, PartSetHeader]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<block::Id, Encoding::Error> {
        let product![hash, part_set_header] = encoding.decode_mut(buffer)?;
        Ok(block::Id {
            hash,
            part_set_header,
        })
    }
}

pub struct EncodePartSetHeader;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, PartSetHeader> for EncodePartSetHeader
where
    Encoding: CanEncodeMut<Strategy, Product![u32, TmHash]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &PartSetHeader,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&product![value.total, value.hash], buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, PartSetHeader> for EncodePartSetHeader
where
    Encoding: CanDecodeMut<Strategy, Product![u32, TmHash]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<PartSetHeader, Encoding::Error> {
        let product![total, hash] = encoding.decode_mut(buffer)?;
        PartSetHeader::new(total, hash)
            .map_err(|_| Encoding::raise_error("invalid part set header"))
    }
}

pub struct EncodeSignature;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Signature> for EncodeSignature
where
    Encoding: CanEncodeMut<Strategy, Vec<u8>>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Signature,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.clone().into_bytes(), buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Signature> for EncodeSignature
where
    Encoding: CanDecodeMut<Strategy, Vec<u8>> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Signature, Encoding::Error> {
        let value = encoding.decode_mut(buffer)?;
        value
            .try_into()
            .map_err(|_| Encoding::raise_error("invalid signature"))
    }
}

pub struct EncodeProtoTimestamp;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ProtoTimestamp> for EncodeProtoTimestamp
where
    Encoding: CanEncodeMut<Strategy, Product![i64, i32]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &ProtoTimestamp,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let ProtoTimestamp { seconds, nanos } = *value;
        encoding.encode_mut(&product![seconds, nanos], buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ProtoTimestamp> for EncodeProtoTimestamp
where
    Encoding: CanDecodeMut<Strategy, Product![i64, i32]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ProtoTimestamp, Encoding::Error> {
        let product![seconds, nanos] = encoding.decode_mut(buffer)?;
        Ok(ProtoTimestamp { seconds, nanos })
    }
}

pub struct EncodeCommitBlockIdFlag;

delegate_components! {
    EncodeCommitBlockIdFlag {
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]: EncodeVariantFrom<Self>,
    }
}

impl TransformerRef for EncodeCommitBlockIdFlag {
    type From = BlockIdFlag;
    type To<'a> = Sum![(), (), (), ()];

    fn transform<'a>(from: &'a BlockIdFlag) -> Self::To<'a> {
        match from {
            BlockIdFlag::Absent => Either::Right(Either::Left(())),
            BlockIdFlag::Commit => Either::Right(Either::Right(Either::Left(()))),
            BlockIdFlag::Nil => Either::Right(Either::Right(Either::Right(Either::Left(())))),
        }
    }
}

impl Transformer for EncodeCommitBlockIdFlag {
    type From = Sum![(), (), (), ()];
    type To = BlockIdFlag;

    fn transform(from: Self::From) -> Self::To {
        match from {
            Either::Left(()) => unreachable!(),
            Either::Right(Either::Left(())) => BlockIdFlag::Absent,
            Either::Right(Either::Right(Either::Left(()))) => BlockIdFlag::Commit,
            Either::Right(Either::Right(Either::Right(Either::Left(())))) => BlockIdFlag::Nil,
            Either::Right(Either::Right(Either::Right(Either::Right(v)))) => match v {},
        }
    }
}

pub struct EncodeCommitSig;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, CommitSig> for EncodeCommitSig
where
    Encoding: CanEncodeMut<Strategy, Product![BlockIdFlag, account::Id, ProtoTimestamp, Signature]>
        + CanEncodeMut<Strategy, Product![BlockIdFlag, Vec<u8>, ProtoTimestamp, Vec<u8>]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &CommitSig,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        match value {
            CommitSig::BlockIdFlagAbsent => encoding.encode_mut(
                &product![BlockIdFlag::Absent, Vec::new(), ZERO_TIMESTAMP, Vec::new()],
                buffer,
            )?,
            CommitSig::BlockIdFlagCommit {
                validator_address,
                timestamp,
                signature,
            } => encoding.encode_mut(
                &product![
                    BlockIdFlag::Commit,
                    *validator_address,
                    (*timestamp).into(),
                    signature.clone().unwrap()
                ],
                buffer,
            )?,
            CommitSig::BlockIdFlagNil {
                validator_address,
                timestamp,
                signature,
            } => encoding.encode_mut(
                &product![
                    BlockIdFlag::Nil,
                    *validator_address,
                    (*timestamp).into(),
                    signature.clone().unwrap()
                ],
                buffer,
            )?,
#[derive(Debug, Clone, HasField, HasFields)]
pub struct CometUpdateHeader {
    pub trusted_height: Height,
    pub target_height: Height,
    pub time: Timestamp,
    pub root: [u32; 8],
}

impl From<TendermintHeader> for CometUpdateHeader {
    fn from(header: TendermintHeader) -> Self {
        let trusted_height = Height {
            revision_number: header.trusted_height.revision_number(),
            revision_height: header.trusted_height.revision_height(),
        };

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, CommitSig> for EncodeCommitSig
where
    Encoding: CanDecodeMut<Strategy, Product![BlockIdFlag, account::Id, ProtoTimestamp, Signature]>
        + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<CommitSig, Encoding::Error> {
        let product![block_id_flag, validator_address, proto_timestamp, signature] =
            encoding.decode_mut(buffer)?;

        let timestamp = proto_timestamp
            .try_into()
            .map_err(|_| Encoding::raise_error("invalid timestamp"))?;

        let value = match block_id_flag {
            BlockIdFlag::Absent => CommitSig::BlockIdFlagAbsent,
            BlockIdFlag::Commit => CommitSig::BlockIdFlagCommit {
                validator_address,
                timestamp,
                signature: Some(signature),
            },
            BlockIdFlag::Nil => CommitSig::BlockIdFlagNil {
                validator_address,
                timestamp,
                signature: Some(signature),
            },
        };

        Ok(value)
    }
}

pub struct EncodeValidatorSet;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, validator::Set> for EncodeValidatorSet
where
    Encoding: CanEncodeMut<Strategy, Product![Vec<validator::Info>, validator::Info, vote::Power]>
        + CanRaiseAsyncError<&'static str>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &validator::Set,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(
            &product![
                value.validators().clone(),
                value.proposer().clone().ok_or_else(|| {
                    Encoding::raise_error("proposer not found in validator set")
                })?,
                value.total_voting_power()
            ],
            buffer,
        )?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, validator::Set> for EncodeValidatorSet
where
    Encoding: CanDecodeMut<Strategy, Product![Vec<validator::Info>, validator::Info, vote::Power]>
        + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<validator::Set, Encoding::Error> {
        let product![validators, proposer, total_voting_power] = encoding.decode_mut(buffer)?;

        let validator_set = validator::Set::new(validators, Some(proposer));

        assert!(validator_set.total_voting_power() == total_voting_power);

        Ok(validator_set)
    }
}

pub struct EncodeVotePower;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, vote::Power> for EncodeVotePower
where
    Encoding: CanEncodeMut<Strategy, u64>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &vote::Power,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.value(), buffer)?;
        Ok(())
    }
}
#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, vote::Power> for EncodeVotePower
where
    Encoding: CanDecodeMut<Strategy, u64> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<vote::Power, Encoding::Error> {
        let value = encoding.decode_mut(buffer)?;
        value
            .try_into()
            .map_err(|_| Encoding::raise_error("invalid power"))
    }
}

pub struct EncodeValidator;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, validator::Info> for EncodeValidator
where
    Encoding:
        CanEncodeMut<Strategy, Product![account::Id, PublicKey, vote::Power, ProposerPriority]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &validator::Info,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(
            &product![
                value.address,
                value.pub_key,
                value.power,
                value.proposer_priority,
            ],
            buffer,
        )?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, validator::Info> for EncodeValidator
where
    Encoding: CanDecodeMut<Strategy, Product![account::Id, PublicKey, vote::Power, ProposerPriority]>
        + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<validator::Info, Encoding::Error> {
        let product![address, pub_key, power, proposer_priority] = encoding.decode_mut(buffer)?;
        Ok(validator::Info {
            address,
            pub_key,
            power,
            name: None,
            proposer_priority,
        })
    }
}

pub struct EncodeAccountId;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, account::Id> for EncodeAccountId
where
    Encoding: CanEncodeMut<Strategy, Vec<u8>>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &account::Id,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.as_bytes().to_vec(), buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, account::Id> for EncodeAccountId
where
    Encoding: CanDecodeMut<Strategy, Vec<u8>> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<account::Id, Encoding::Error> {
        let value = encoding.decode_mut(buffer)?;
        Ok(account::Id::new(value.try_into().map_err(|_| {
            Encoding::raise_error("invalid account id")
        })?))
    }
}

pub struct EncodeProposerPriority;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ProposerPriority> for EncodeProposerPriority
where
    Encoding: CanEncodeMut<Strategy, i64>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &ProposerPriority,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.value(), buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ProposerPriority> for EncodeProposerPriority
where
    Encoding: CanDecodeMut<Strategy, i64> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ProposerPriority, Encoding::Error> {
        encoding.decode_mut(buffer)?;
        let value = encoding.decode_mut(buffer)?;
        Ok(value.into())
    }
}

pub struct EncodePublicKey;

delegate_components! {
    EncodePublicKey {
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]: EncodeVariantFrom<Self>,
    }
}

impl TransformerRef for EncodePublicKey {
    type From = PublicKey;
    type To<'a> = Sum![Vec<u8>, Vec<u8>];

    fn transform<'a>(from: &'a PublicKey) -> Self::To<'a> {
        match from {
            PublicKey::Ed25519(key) => Either::Left(key.as_bytes().to_vec()),
            PublicKey::Secp256k1(key) => unimplemented!(),
            &_ => unimplemented!(),
        }
    }
}

impl Transformer for EncodePublicKey {
    type From = Sum![Vec<u8>, Vec<u8>];
    type To = PublicKey;

    fn transform(value: Self::From) -> PublicKey {
        match value {
            Either::Left(key) => PublicKey::Ed25519(key.as_slice().try_into().unwrap()),
            Either::Right(Either::Left(key)) => unimplemented!(),
            Either::Right(Either::Right(v)) => match v {},
        }
    }
}
