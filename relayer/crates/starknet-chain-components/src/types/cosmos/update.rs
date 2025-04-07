use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, MutDecoder, MutDecoderComponent,
};
use hermes_encoding_components::traits::encode_mut::{
    CanEncodeMut, MutEncoder, MutEncoderComponent,
};
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
use ibc::clients::tendermint::types::Header as TendermintLcHeader;
use ibc::core::client::types::Height as IbcHeight;
use ibc_proto::google::protobuf::Timestamp as ProtoTimestamp;
use tendermint::block::parts::Header as PartSetHeader;
use tendermint::block::signed_header::SignedHeader;
use tendermint::block::{Commit, CommitSig, Header as TmHeader};
use tendermint::hash::Hash as TmHash;
use tendermint::validator::ProposerPriority;
use tendermint::{account, block, validator, vote, PublicKey, Signature};

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

pub struct EncodeCommitSig;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, CommitSig> for EncodeCommitSig
where
    Encoding: CanEncodeMut<
            Strategy,
            Product![Sum![(), (), (), ()], account::Id, ProtoTimestamp, Signature],
        > + CanEncodeMut<Strategy, Product![Sum![(), (), (), ()], Vec<u8>, ProtoTimestamp, Vec<u8>]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &CommitSig,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        match value {
            CommitSig::BlockIdFlagAbsent => encoding.encode_mut(
                &product![
                    Either::Right(Either::Left(())),
                    Vec::new(),
                    ZERO_TIMESTAMP,
                    Vec::new()
                ],
                buffer,
            )?,
            CommitSig::BlockIdFlagCommit {
                validator_address,
                timestamp,
                signature,
            } => encoding.encode_mut(
                &product![
                    Either::Right(Either::Right(Either::Left(()))),
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
                    Either::Right(Either::Right(Either::Right(Either::Left(())))),
                    *validator_address,
                    (*timestamp).into(),
                    signature.clone().unwrap()
                ],
                buffer,
            )?,
        };

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, CommitSig> for EncodeCommitSig
where
    Encoding: CanDecodeMut<
            Strategy,
            Product![Sum![(), (), (), ()], account::Id, ProtoTimestamp, Signature],
        > + CanRaiseAsyncError<&'static str>,
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
            Either::Left(()) => unreachable!(),
            Either::Right(Either::Left(())) => CommitSig::BlockIdFlagAbsent,
            Either::Right(Either::Right(Either::Left(()))) => CommitSig::BlockIdFlagCommit {
                validator_address,
                timestamp,
                signature: Some(signature),
            },
            Either::Right(Either::Right(Either::Right(Either::Left(())))) => {
                CommitSig::BlockIdFlagNil {
                    validator_address,
                    timestamp,
                    signature: Some(signature),
                }
            }
            Either::Right(Either::Right(Either::Right(Either::Right(v)))) => match v {},
        };

        Ok(value)
    }
}

pub struct EncodeValidatorSet;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, validator::Set> for EncodeValidatorSet
where
    Encoding: CanEncodeMut<
        Strategy,
        Product![Vec<validator::Info>, Option<validator::Info>, vote::Power],
    >,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &validator::Set,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(
            &product![
                value.validators().clone(),
                value.proposer().clone(),
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
    Encoding: CanDecodeMut<Strategy, Product![Vec<validator::Info>, Option<validator::Info>, vote::Power]>
        + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<validator::Set, Encoding::Error> {
        let product![validators, proposer, total_voting_power] = encoding.decode_mut(buffer)?;

        let validator_set = validator::Set::new(validators, proposer);

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
    Encoding: CanEncodeMut<Strategy, (account::Id, PublicKey, vote::Power, ProposerPriority)>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &validator::Info,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(
            &(
                value.address,
                value.pub_key,
                value.power,
                value.proposer_priority,
            ),
            buffer,
        )?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, validator::Info> for EncodeValidator
where
    Encoding: CanDecodeMut<Strategy, (account::Id, PublicKey, vote::Power, ProposerPriority)>
        + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<validator::Info, Encoding::Error> {
        let (address, pub_key, power, proposer_priority) = encoding.decode_mut(buffer)?;
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
    Encoding: CanEncodeMut<Strategy, [u8; account::LENGTH]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &account::Id,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.as_bytes().to_vec().try_into().unwrap(), buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, account::Id> for EncodeAccountId
where
    Encoding: CanDecodeMut<Strategy, [u8; account::LENGTH]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<account::Id, Encoding::Error> {
        let value = encoding.decode_mut(buffer)?;
        Ok(account::Id::new(value))
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
