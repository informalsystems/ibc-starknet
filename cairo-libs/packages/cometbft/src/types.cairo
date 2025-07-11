pub use canonical_vote_impl::CanonicalVoteAsProtoMessage;
use cometbft::errors::CometErrors;
use cometbft::utils::Fraction;
use core::sha256::compute_sha256_byte_array;
use protobuf::primitives::array::{ByteArrayAsProtoMessage, BytesAsProtoMessage};
use protobuf::primitives::numeric::{
    BoolAsProtoMessage, I32AsProtoMessage, I64AsProtoMessage, U64AsProtoMessage,
};
use protobuf::types::message::{
    DecodeContext, DecodeContextImpl, DecodeContextTrait, EncodeContext, EncodeContextImpl,
    ProtoCodecImpl, ProtoMessage, ProtoName, ProtoOneof,
};
use protobuf::types::tag::{ProtobufTag, WireType};
use protobuf::types::wkt::{Duration, Timestamp};
use crate::ed25519::GaragaEd25519Verifier as Ed25519Verifier;
use crate::light_client::Header as LcHeader;

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub struct Consensus {
    pub block: u64,
    pub app: u64,
}

impl ConsensusAsProtoMessage of ProtoMessage<Consensus> {
    fn encode_raw(self: @Consensus, ref context: EncodeContext) {
        context.encode_field(1, self.block);
        context.encode_field(2, self.app);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Consensus> {
        let block = context.decode_field(1)?;
        let app = context.decode_field(2)?;
        Option::Some(Consensus { block, app })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ConsensusAsProtoName of ProtoName<Consensus> {
    fn type_url() -> ByteArray {
        "Consensus"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct PartSetHeader {
    pub total: u32,
    pub hash: Array<u8>,
}

impl PartSetHeaderAsProtoMessage of ProtoMessage<PartSetHeader> {
    fn encode_raw(self: @PartSetHeader, ref context: EncodeContext) {
        context.encode_field(1, self.total);
        context.encode_field(2, self.hash);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<PartSetHeader> {
        let total = context.decode_field(1)?;
        let hash = context.decode_field(2)?;
        Option::Some(PartSetHeader { total, hash })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl PartSetHeaderAsProtoName of ProtoName<PartSetHeader> {
    fn type_url() -> ByteArray {
        "PartSetHeader"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct BlockId {
    pub hash: Array<u8>,
    pub part_set_header: PartSetHeader,
}

impl BlockIdAsProtoMessage of ProtoMessage<BlockId> {
    fn encode_raw(self: @BlockId, ref context: EncodeContext) {
        context.encode_field(1, self.hash);
        context.encode_field(2, self.part_set_header);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<BlockId> {
        let hash = context.decode_field(1)?;
        let part_set_header = context.decode_field(2)?;
        Option::Some(BlockId { hash, part_set_header })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl BlockIdAsProtoName of ProtoName<BlockId> {
    fn type_url() -> ByteArray {
        "BlockId"
    }
}


#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct Header {
    pub version: Consensus,
    pub chain_id: ByteArray,
    pub height: u64,
    pub time: Timestamp,
    pub last_block_id: BlockId,
    pub last_commit_hash: Array<u8>,
    pub data_hash: Array<u8>,
    pub validators_hash: Array<u8>,
    pub next_validators_hash: Array<u8>,
    pub consensus_hash: Array<u8>,
    pub app_hash: Array<u8>,
    pub last_results_hash: Array<u8>,
    pub evidence_hash: Array<u8>,
    pub proposer_address: Array<u8>,
}

impl HeaderAsProtoMessage of ProtoMessage<Header> {
    fn encode_raw(self: @Header, ref context: EncodeContext) {
        context.encode_field(1, self.version);
        context.encode_field(2, self.chain_id);
        context.encode_field(3, self.height);
        context.encode_field(4, self.time);
        context.encode_field(5, self.last_block_id);
        context.encode_field(6, self.last_commit_hash);
        context.encode_field(7, self.data_hash);
        context.encode_field(8, self.validators_hash);
        context.encode_field(9, self.next_validators_hash);
        context.encode_field(10, self.consensus_hash);
        context.encode_field(11, self.app_hash);
        context.encode_field(12, self.last_results_hash);
        context.encode_field(13, self.evidence_hash);
        context.encode_field(14, self.proposer_address);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Header> {
        let version = context.decode_field(1)?;
        let chain_id = context.decode_field(2)?;
        let height = context.decode_field(3)?;
        let time = context.decode_field(4)?;
        let last_block_id = context.decode_field(5)?;
        let last_commit_hash = context.decode_field(6)?;
        let data_hash = context.decode_field(7)?;
        let validators_hash = context.decode_field(8)?;
        let next_validators_hash = context.decode_field(9)?;
        let consensus_hash = context.decode_field(10)?;
        let app_hash = context.decode_field(11)?;
        let last_results_hash = context.decode_field(12)?;
        let evidence_hash = context.decode_field(13)?;
        let proposer_address = context.decode_field(14)?;
        Option::Some(
            Header {
                version,
                chain_id,
                height,
                time,
                last_block_id,
                last_commit_hash,
                data_hash,
                validators_hash,
                next_validators_hash,
                consensus_hash,
                app_hash,
                last_results_hash,
                evidence_hash,
                proposer_address,
            },
        )
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl HeaderAsProtoName of ProtoName<Header> {
    fn type_url() -> ByteArray {
        "Header"
    }
}

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub enum BlockIdFlag {
    #[default]
    Unknown,
    Absent,
    Commit,
    Nil,
}

pub impl BlockIdFlagIntoU32 of Into<@BlockIdFlag, u32> {
    fn into(self: @BlockIdFlag) -> u32 {
        match self {
            BlockIdFlag::Unknown => 0,
            BlockIdFlag::Absent => 1,
            BlockIdFlag::Commit => 2,
            BlockIdFlag::Nil => 3,
        }
    }
}

pub impl U32TryIntoBlockIdFlag of TryInto<u32, BlockIdFlag> {
    fn try_into(self: u32) -> Option<BlockIdFlag> {
        match self {
            0 => Option::Some(BlockIdFlag::Unknown),
            1 => Option::Some(BlockIdFlag::Absent),
            2 => Option::Some(BlockIdFlag::Commit),
            3 => Option::Some(BlockIdFlag::Nil),
            _ => Option::None,
        }
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct CommitSig {
    pub block_id_flag: BlockIdFlag,
    pub validator_address: AccountId,
    pub timestamp: Timestamp,
    pub signature: Array<u8>,
}

impl CommitSigAsProtoMessage of ProtoMessage<CommitSig> {
    fn encode_raw(self: @CommitSig, ref context: EncodeContext) {
        context.encode_enum(1, self.block_id_flag);
        context.encode_field(2, self.validator_address);
        context.encode_field(3, self.timestamp);
        context.encode_field(4, self.signature);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<CommitSig> {
        let block_id_flag = context.decode_enum(1)?;
        let validator_address = context.decode_field(2)?;
        let timestamp = context.decode_field(3)?;
        let signature = context.decode_field(4)?;
        Option::Some(CommitSig { block_id_flag, validator_address, timestamp, signature })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl CommitSigAsProtoName of ProtoName<CommitSig> {
    fn type_url() -> ByteArray {
        "CommitSig"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct Commit {
    pub height: i64,
    pub round: i32,
    pub block_id: BlockId,
    pub signatures: Array<CommitSig>,
}

impl CommitAsProtoMessage of ProtoMessage<Commit> {
    fn encode_raw(self: @Commit, ref context: EncodeContext) {
        context.encode_field(1, self.height);
        context.encode_field(2, self.round);
        context.encode_field(3, self.block_id);
        context.encode_repeated_field(4, self.signatures);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Commit> {
        let height = context.decode_field(1)?;
        let round = context.decode_field(2)?;
        let block_id = context.decode_field(3)?;
        let signatures = context.decode_repeated_field(4)?;
        Option::Some(Commit { height, round, block_id, signatures })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl CommitAsProtoName of ProtoName<Commit> {
    fn type_url() -> ByteArray {
        "Commit"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct SignedHeader {
    pub header: Header,
    pub commit: Commit,
}

impl SignedHeaderAsProtoMessage of ProtoMessage<SignedHeader> {
    fn encode_raw(self: @SignedHeader, ref context: EncodeContext) {
        context.encode_field(1, self.header);
        context.encode_field(2, self.commit);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<SignedHeader> {
        let header = context.decode_field(1)?;
        let commit = context.decode_field(2)?;
        Option::Some(SignedHeader { header, commit })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl SignedHeaderAsProtoName of ProtoName<SignedHeader> {
    fn type_url() -> ByteArray {
        "SignedHeader"
    }
}

#[derive(Drop, Default, Debug, Clone, PartialEq, Serde)]
pub struct PublicKey {
    pub sum: Sum,
}

#[generate_trait]
pub impl PublicKeyImpl of PublicKeyTrait {
    fn verify(self: @PublicKey, msg: Span<u8>, signature: Span<u8>, hint: Span<felt252>) {
        match self.sum {
            Sum::Ed25519(pk) => Ed25519Verifier::assert_signature(msg, signature, pk.span(), hint),
            _ => core::panic_with_felt252(CometErrors::UNSUPPORTED_PUBKEY_TYPE),
        }
    }

    fn address(self: @PublicKey) -> AccountId {
        match self.sum {
            Sum::Ed25519(pk) => {
                let pub_key_byte_array = {
                    let mut val = "";
                    let mut span = pk.span();

                    while let Some(b) = span.pop_front() {
                        val.append_byte(*b);
                    }

                    val
                };

                let hash = compute_sha256_byte_array(@pub_key_byte_array);
                let span = hash.span();

                let mut id: Array<u8> = array![];
                let limit: usize = 5;

                for i in 0..limit {
                    let value = *span[i];
                    let byte1 = (value & 0xFF000000) / 0x1000000;
                    let byte2 = (value & 0x00FF0000) / 0x0010000;
                    let byte3 = (value & 0x0000FF00) / 0x0000100;
                    let byte4 = value & 0x000000FF;
                    id.append(byte1.try_into().unwrap());
                    id.append(byte2.try_into().unwrap());
                    id.append(byte3.try_into().unwrap());
                    id.append(byte4.try_into().unwrap());
                }

                id.try_into().unwrap() // Never fails as length is 20.
            },
            _ => core::panic_with_felt252(CometErrors::UNSUPPORTED_PUBKEY_TYPE),
        }
    }
}

impl PublicKeyAsProtoMessage of ProtoMessage<PublicKey> {
    fn encode_raw(self: @PublicKey, ref context: EncodeContext) {
        context.encode_oneof(self.sum)
    }

    fn decode_raw(ref context: DecodeContext) -> Option<PublicKey> {
        let sum = context.decode_oneof()?;
        Option::Some(PublicKey { sum })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Drop, Default, Debug, Clone, PartialEq, Serde)]
pub enum Sum {
    #[default]
    Ed25519: Array<u8>,
    Secp256k1: Array<u8>,
}

impl SumAsProtoOneof of ProtoOneof<Sum> {
    fn encode_raw(self: @Sum, ref context: EncodeContext) -> ProtobufTag {
        match self {
            Sum::Ed25519(k) => {
                k.encode_raw(ref context);
                let wire_type = ProtoMessage::<Array<u8>>::wire_type();
                ProtobufTag { field_number: 1, wire_type }
            },
            Sum::Secp256k1(k) => {
                k.encode_raw(ref context);
                let wire_type = ProtoMessage::<Array<u8>>::wire_type();
                ProtobufTag { field_number: 2, wire_type }
            },
        }
    }

    fn decode_raw(ref context: DecodeContext, tag: u8) -> Option<Sum> {
        match tag {
            0 => Option::None,
            1 => {
                let sum = context.decode_field(1)?;
                Option::Some(Sum::Ed25519(sum))
            },
            2 => {
                let sum = context.decode_field(2)?;
                Option::Some(Sum::Secp256k1(sum))
            },
            _ => Option::None,
        }
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct Validator {
    pub address: AccountId,
    pub pub_key: PublicKey,
    pub voting_power: u64,
    pub proposer_priority: i64,
}

#[generate_trait]
pub impl ValidatorImpl of ValidatorTrait {
    fn validate_id(self: @Validator) {
        assert(self.address == @self.pub_key.address(), 'invalid validator ID');
    }

    fn verify_signature(
        self: @Validator, sign_bytes: Span<u8>, signature: Span<u8>, hints: Span<felt252>,
    ) {
        self.pub_key.verify(sign_bytes, signature, hints);
    }
}

impl ValidatorAsProtoMessage of ProtoMessage<Validator> {
    fn encode_raw(self: @Validator, ref context: EncodeContext) {
        context.encode_field(1, self.address);
        context.encode_field(2, self.pub_key);
        context.encode_field(3, self.voting_power);
        context.encode_field(4, self.proposer_priority);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Validator> {
        let address = context.decode_field(1)?;
        let pub_key = context.decode_field(2)?;
        let voting_power = context.decode_field(3)?;
        let proposer_priority = context.decode_field(4)?;
        Option::Some(Validator { address, pub_key, voting_power, proposer_priority })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ValidatorAsProtoName of ProtoName<Validator> {
    fn type_url() -> ByteArray {
        "Validator"
    }
}


#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct SimpleValidator {
    pub pub_key: PublicKey,
    pub voting_power: u64,
}

impl SimpleValidatorAsProtoMessage of ProtoMessage<SimpleValidator> {
    fn encode_raw(self: @SimpleValidator, ref context: EncodeContext) {
        context.encode_field(1, self.pub_key);
        context.encode_field(2, self.voting_power);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<SimpleValidator> {
        let pub_key = context.decode_field(1)?;
        let voting_power = context.decode_field(2)?;
        Option::Some(SimpleValidator { pub_key, voting_power })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl SimpleValidatorAsProtoName of ProtoName<SimpleValidator> {
    fn type_url() -> ByteArray {
        "SimpleValidator"
    }
}

impl ValidatorToSimpleValidator of Into<Validator, SimpleValidator> {
    fn into(self: Validator) -> SimpleValidator {
        SimpleValidator { pub_key: self.pub_key, voting_power: self.voting_power }
    }
}


#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct ValidatorSet {
    pub validators: Array<Validator>,
    pub proposer: Validator,
    pub total_voting_power: u64,
}

#[generate_trait]
pub impl ValidatorSetImpl of ValidatorSetTrait {
    fn total_power(self: @ValidatorSet) -> u64 {
        let mut power = 0;
        for v in self.validators.span() {
            power += v.voting_power.deref();
        }
        power
    }
}

impl ValidatorSetAsProtoMessage of ProtoMessage<ValidatorSet> {
    fn encode_raw(self: @ValidatorSet, ref context: EncodeContext) {
        context.encode_repeated_field(1, self.validators);
        context.encode_field(2, self.proposer);
        context.encode_field(3, self.total_voting_power);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<ValidatorSet> {
        let validators = context.decode_repeated_field(1)?;
        let proposer = context.decode_field(2)?;
        let total_voting_power = context.decode_field(3)?;
        Option::Some(ValidatorSet { validators, proposer, total_voting_power })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ValidatorSetAsProtoName of ProtoName<ValidatorSet> {
    fn type_url() -> ByteArray {
        "ValidatorSet"
    }
}

/// Size of an account ID in bytes.
pub const LENGTH: u32 = 20;

#[derive(Drop, Clone)]
pub struct AccountId {
    id: [u8; LENGTH],
}

pub impl ArrayU8TryIntoAccountId of TryInto<Array<u8>, AccountId> {
    fn try_into(self: Array<u8>) -> Option<AccountId> {
        if self.len() != LENGTH {
            return Option::None;
        }
        let id = [
            *self[0], *self[1], *self[2], *self[3], *self[4], *self[5], *self[6], *self[7],
            *self[8], *self[9], *self[10], *self[11], *self[12], *self[13], *self[14], *self[15],
            *self[16], *self[17], *self[18], *self[19],
        ];
        Option::Some(AccountId { id })
    }
}

impl AccountIdDebug of core::fmt::Debug<AccountId> {
    fn fmt(self: @AccountId, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let id: Array<u8> = self.id.span().into();
        f.buffer.append(@format!("{id:?}"));
        Result::Ok(())
    }
}

impl AccountIdPartialEq of core::traits::PartialEq<AccountId> {
    fn eq(lhs: @AccountId, rhs: @AccountId) -> bool {
        let mut lhs_span = lhs.id.span();
        let mut rhs_span = rhs.id.span();

        if lhs_span.len() != rhs_span.len() {
            return false;
        }

        let len = lhs_span.len();

        if len == 0 {
            return true;
        }

        let mut result = true;

        while let (Some(lhs), Some(rhs)) = (lhs_span.pop_front(), rhs_span.pop_front()) {
            if lhs != rhs {
                result = false;
                break;
            }
        }

        return result;
    }
}

impl AccountIdDefault of core::traits::Default<AccountId> {
    fn default() -> AccountId {
        AccountId { id: [0_u8; LENGTH] }
    }
}

impl AccountIdAsProtoMessage of ProtoMessage<AccountId> {
    fn encode_raw(self: @AccountId, ref context: EncodeContext) {
        let bytes: Array<u8> = self.id.span().into();
        BytesAsProtoMessage::encode_raw(@bytes, ref context);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<AccountId> {
        let bytes = BytesAsProtoMessage::decode_raw(ref context)?;
        bytes.try_into()
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl AccountIdSerde of Serde<AccountId> {
    fn serialize(self: @AccountId, ref output: Array<felt252>) {
        let mut id = self.id.span().into();
        Serde::<Array<u8>>::serialize(@id, ref output);
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<AccountId> {
        Serde::<Array<u8>>::deserialize(ref serialized)?.try_into()
    }
}

#[derive(Drop, Debug, Clone, PartialEq)]
pub struct NonAbsentCommitVotes {
    votes: Array<(NonAbsentCommitVote, Array<felt252>)>,
}

#[generate_trait]
pub impl NonAbsentCommitVotesImpl of NonAbsentCommitVotesTrait {
    fn new(
        signed_header: SignedHeader, mut signature_hints: Array<Array<felt252>>,
    ) -> NonAbsentCommitVotes {
        let mut votes = ArrayTrait::new();

        let commit = signed_header.commit;

        let mut signatures = commit.signatures.span();
        let mut signature_hints = signature_hints.span();

        assert(
            signatures.len() == signature_hints.len(), CometErrors::INVALID_SIGNATURE_HINTS_LENGTH,
        );

        while let (Some(signature), Some(signature_hint)) =
            (signatures.pop_front(), signature_hints.pop_front()) {
            let block_id_flag = signature.block_id_flag;
            if block_id_flag == @BlockIdFlag::Commit {
                let signed_vote = SignedVote {
                    vote: CanonicalVote {
                        vote_type: VoteType::Precommit,
                        height: commit.height.clone().try_into().unwrap(),
                        round: commit.round.clone().try_into().unwrap(),
                        block_id: commit.block_id.clone(),
                        timestamp: signature.timestamp.clone(),
                        chain_id: signed_header.header.chain_id.clone(),
                    },
                    validator_address: signature.validator_address.clone(),
                    signature: signature.signature.clone(),
                };

                let verified = false;
                let non_absent_commit_vote = NonAbsentCommitVote { signed_vote, verified };

                votes.append((non_absent_commit_vote, signature_hint.clone()));
            }
        }

        NonAbsentCommitVotes { votes }
    }

    fn has_voted(self: @NonAbsentCommitVotes, validator: @Validator) -> bool {
        let mut votes_span = self.votes.span();
        let mut result = false;
        while let Option::Some((ith_vote, signature_hints)) = votes_span.pop_front() {
            if ith_vote.validator_id() == validator.address {
                if *(ith_vote.verified) {
                    result = false;
                    break;
                }

                let signed_vote = ith_vote.signed_vote;

                let signature = signed_vote.signature;
                let canonical_vote = signed_vote.vote;

                let signed_bytes = ProtoCodecImpl::encode_with_length(canonical_vote);

                validator.validate_id();

                validator
                    .verify_signature(
                        signed_bytes.span(), signature.span(), signature_hints.span(),
                    );

                // TODO: set verified field to true

                result = true;
                break;
            }
        }
        return result;
    }
}

#[derive(Drop, Debug, Clone, PartialEq)]
pub struct NonAbsentCommitVote {
    signed_vote: SignedVote,
    /// Flag indicating whether the signature has already been verified.
    verified: bool,
}

#[generate_trait]
pub impl NonAbsentCommitVoteImpl of NonAbsentCommitVoteTrait {
    fn validator_id(self: @NonAbsentCommitVote) -> @AccountId {
        self.signed_vote.validator_address
    }
}

#[derive(Drop, Debug, Clone, PartialEq)]
pub struct SignedVote {
    vote: CanonicalVote,
    validator_address: AccountId,
    signature: Array<u8> // TODO: whether to define a Signature type?
}

#[derive(Drop, Debug, Clone, PartialEq, Default)]
pub struct CanonicalVote {
    /// Type of vote (prevote or precommit)
    pub vote_type: VoteType,
    /// Block height
    pub height: i64,
    /// Round
    pub round: i64,
    pub block_id: BlockId,
    /// Timestamp
    pub timestamp: Timestamp,
    /// Chain ID
    pub chain_id: ByteArray,
}

/// Type of votes
#[derive(Drop, Debug, Clone, PartialEq, Default)]
pub enum VoteType {
    #[default]
    Unknown,
    /// Votes for blocks which validators observe are valid for a given round
    Prevote,
    /// Votes to commit to a particular block for a given round
    Precommit,
    Proposal,
}

pub impl VoteTypeIntoU32 of Into<@VoteType, u32> {
    fn into(self: @VoteType) -> u32 {
        match self {
            VoteType::Unknown => 0,
            VoteType::Prevote => 1,
            VoteType::Precommit => 2,
            VoteType::Proposal => 32,
        }
    }
}

pub impl U32TryIntoVoteType of TryInto<u32, VoteType> {
    fn try_into(self: u32) -> Option<VoteType> {
        if self == 0 {
            return Option::Some(VoteType::Unknown);
        } else if self == 1 {
            return Option::Some(VoteType::Prevote);
        } else if self == 2 {
            return Option::Some(VoteType::Precommit);
        } else if self == 32 {
            return Option::Some(VoteType::Proposal);
        } else {
            return Option::None;
        }
    }
}

#[derive(Drop, Debug, Clone, Serde)]
pub struct UntrustedBlockState {
    pub signed_header: SignedHeader,
    pub validators: ValidatorSet,
    pub next_validators: ValidatorSet,
}

#[generate_trait]
pub impl UntrustedBlockStateImpl of UntrustedBlockStateTrait {
    fn height(self: @UntrustedBlockState) -> @u64 {
        self.signed_header.header.height
    }
}

impl HeaderToUntrustedBlockState of Into<LcHeader, UntrustedBlockState> {
    fn into(self: LcHeader) -> UntrustedBlockState {
        UntrustedBlockState {
            signed_header: self.signed_header,
            validators: self.validator_set,
            next_validators: self.trusted_validator_set,
        }
    }
}

#[derive(Drop, Debug, Clone, Serde)]
pub struct TrustedBlockState {
    pub chain_id: ByteArray,
    pub header_time: Timestamp,
    pub height: u64,
    pub next_validators: ValidatorSet,
    pub next_validators_hash: Array<u8>,
}

#[derive(Drop, Debug, Clone, Serde)]
pub struct Options {
    pub trust_threshold: Fraction,
    pub trusting_period: Duration,
    pub clock_drift: Duration,
}

#[derive(Drop, Debug, Clone, PartialEq, Serde)]
pub struct VotingPowerTally {
    /// Total voting power
    pub total: u64,
    /// Tallied voting power
    pub tallied: u64,
    /// Trust threshold for voting power
    pub trust_threshold: Fraction,
}

#[generate_trait]
pub impl VotingPowerTallyImpl of VotingPowerTallyTrait {
    fn new(total: u64, trust_threshold: Fraction) -> VotingPowerTally {
        VotingPowerTally { total, tallied: 0, trust_threshold }
    }

    fn tally(ref self: VotingPowerTally, power: u64) {
        self.tallied += power;
    }

    fn has_enough_power(self: @VotingPowerTally) -> bool {
        // 0 < numerator < denominator
        assert(@0 < self.trust_threshold.numerator, 'invalid trust threshold');
        assert(
            self.trust_threshold.numerator < self.trust_threshold.denominator,
            'invalid trust threshold',
        );

        // cast to u128 to avoid overflow
        let tally: u128 = (*self.tallied).into();
        let total: u128 = (*self.total).into();
        let numerator: u128 = (*self.trust_threshold.numerator).into();
        let denominator: u128 = (*self.trust_threshold.denominator).into();

        // tally / total >= numerator / denominator
        tally * denominator >= total * numerator
    }
}

pub mod canonical_vote_impl {
    // hack to avoid multiple impls for i64 - I64AsProtoMessage and SFixed64AsProtoMessage

    use protobuf::primitives::array::ByteArrayAsProtoMessage;
    use protobuf::primitives::numeric::SFixed64AsProtoMessage;
    use protobuf::types::message::{
        DecodeContext, DecodeContextImpl, DecodeContextTrait, EncodeContext, EncodeContextImpl,
        ProtoCodecImpl, ProtoMessage,
    };
    use protobuf::types::tag::WireType;
    use super::CanonicalVote;

    pub impl CanonicalVoteAsProtoMessage of ProtoMessage<CanonicalVote> {
        fn encode_raw(self: @CanonicalVote, ref context: EncodeContext) {
            context.encode_enum(1, self.vote_type);
            context.encode_field(2, self.height);
            context.encode_field(3, self.round);
            context.encode_field(4, self.block_id);
            context.encode_field(5, self.timestamp);
            context.encode_field(6, self.chain_id);
        }

        fn decode_raw(ref context: DecodeContext) -> Option<CanonicalVote> {
            let vote_type = context.decode_enum(1)?;
            let height = context.decode_field(2)?;
            let round = context.decode_field(3)?;
            let block_id = context.decode_field(4)?;
            let timestamp = context.decode_field(5)?;
            let chain_id = context.decode_field(6)?;
            Option::Some(CanonicalVote { vote_type, height, round, block_id, timestamp, chain_id })
        }

        fn wire_type() -> WireType {
            WireType::LengthDelimited
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enough_power() {
        let mut tailled_power = VotingPowerTallyImpl::new(
            100, Fraction { numerator: 1, denominator: 2 },
        );

        tailled_power.tally(50);

        assert!(tailled_power.has_enough_power());
    }

    #[test]
    fn test_not_enough_power() {
        let mut tailled_power = VotingPowerTallyImpl::new(
            100, Fraction { numerator: 1, denominator: 3 },
        );

        tailled_power.tally(33);

        assert!(!tailled_power.has_enough_power());
    }

    #[test]
    fn test_big_numbers() {
        let mut tallied_power = VotingPowerTallyImpl::new(
            0xFFFFFFFFFFFFFFFF, Fraction { numerator: 13, denominator: 37 },
        );
        tallied_power.tally(0xFFFFFFFFFFFFFFF0);

        assert!(tallied_power.has_enough_power());
    }

    #[test]
    #[should_panic]
    fn test_enough_power_with_overflow() {
        let mut tailled_power = VotingPowerTallyImpl::new(
            100, Fraction { numerator: 1, denominator: 0 },
        );

        tailled_power.tally(50);

        assert!(!tailled_power.has_enough_power());
    }
}
