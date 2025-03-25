use cometbft::errors::CometErrors;
use cometbft::types::{
    BlockIdFlag, Header, NonAbsentCommitVotes, NonAbsentCommitVotesTrait, Options, SignedHeader,
    SimpleValidator, TrustedBlockState, UntrustedBlockState, UntrustedBlockStateTrait, ValidatorSet,
    ValidatorSetTrait, VotingPowerTally, VotingPowerTallyTrait,
};
use cometbft::utils::{Fraction, TWO_THIRDS};
use core::num::traits::CheckedAdd;
use protobuf::primitives::array::{ByteArrayAsProtoMessage, BytesAsProtoMessage};
use protobuf::primitives::numeric::U64AsProtoMessage;
use protobuf::types::message::ProtoCodecImpl;
use protobuf::types::wkt::{Duration, Timestamp};
use crate::utils::{MerkleHashImpl, u32_8_to_array_u8};

/// Verifies an update header received as the `header` field of `MsgUpdateClient`.
///
/// NOTE: The checks included here are the same as those in
/// [`tendermint-light-client-verifier`](https://github.com/informalsystems/tendermint-rs/blob/6cc391c80ae88615508bd87285571ba130b604c/light-client-verifier/src/verifier.rs#L268-L24).
pub fn verify_update_header(
    untrusted: UntrustedBlockState, trusted: TrustedBlockState, options: Options, now: Timestamp,
) {
    verify_validator_sets(@untrusted);
    validate_against_trusted(@untrusted, @trusted, @options, now);
    verify_header_is_from_past(@untrusted, @options, now);
    verify_commit_against_trusted(untrusted, trusted, options);
}

/// Verifies a misbehaviour header received as the `header` field of `MsgUpdateClient`.
///
/// NOTE: The checks included here are the same as those in
/// [`tendermint-light-client-verifier`](https://github.com/informalsystems/tendermint-rs/blob/6cc31c80ae88615508bd87285571ba130b604c/light-client-verifier/src/verifier.rs#L310-L312)
pub fn verify_misbehaviour_header(
    untrusted: UntrustedBlockState, trusted: TrustedBlockState, options: Options, now: Timestamp,
) {
    verify_validator_sets(@untrusted);
    validate_against_trusted(@untrusted, @trusted, @options, now);
    verify_commit_against_trusted(untrusted, trusted, options)
}

pub fn verify_validator_sets(untrusted: @UntrustedBlockState) {
    validator_sets_match(untrusted.validators, untrusted.signed_header.header.validators_hash);
    // validator_sets_match(
    //     untrusted.next_validators, untrusted.signed_header.header.next_validators_hash,
    // );
    header_matches_commit(
        untrusted.signed_header.header, untrusted.signed_header.commit.block_id.hash,
    );
    valid_commit(untrusted.signed_header, untrusted.validators);
}

pub fn validate_against_trusted(
    untrusted: @UntrustedBlockState, trusted: @TrustedBlockState, options: @Options, now: Timestamp,
) {
    is_within_trust_period(trusted.header_time, options.trusting_period, now);
    is_monotonic_bft_time(untrusted.signed_header.header.time, trusted.header_time);
    is_matching_chain_id(untrusted.signed_header.header.chain_id, trusted.chain_id);
    let trusted_next_height = trusted.height.checked_add(1);
    assert(trusted_next_height.is_some(), CometErrors::OVERFLOWED_BLOCK_HEIGHT);
    if untrusted.height() == @trusted_next_height.unwrap() {
        valid_next_validator_set(
            untrusted.signed_header.header.validators_hash, trusted.next_validators_hash,
        );
    } else {
        is_monotonic_height(untrusted.signed_header.header.height, trusted.height);
    }
}

pub fn verify_header_is_from_past(
    untrusted: @UntrustedBlockState, options: @Options, now: Timestamp,
) {
    is_header_from_past(untrusted.signed_header.header.time, options.clock_drift, now);
}

pub fn validator_sets_match(validator_set: @ValidatorSet, header_validator_hash: @Array<u8>) {
    let mut validator_bytes = array![];

    for validator in validator_set.validators {
        let simple_validator: SimpleValidator = validator.clone().into();
        let bytes = ProtoCodecImpl::encode(@simple_validator);
        validator_bytes.append(bytes);
    }

    let hash = MerkleHashImpl::hash_byte_vectors(validator_bytes.span());

    assert(
        header_validator_hash == @u32_8_to_array_u8(hash), CometErrors::INVALID_VALIDATOR_SET_HASH,
    );
}

pub fn header_matches_commit(header: @Header, commit_hash: @Array<u8>) {
    let header_bytes = array![
        ProtoCodecImpl::encode(header.version),
        ProtoCodecImpl::encode(header.chain_id),
        ProtoCodecImpl::encode(header.height),
        ProtoCodecImpl::encode(header.time),
        ProtoCodecImpl::encode(header.last_block_id),
        ProtoCodecImpl::encode(header.last_commit_hash),
        ProtoCodecImpl::encode(header.data_hash),
        ProtoCodecImpl::encode(header.validators_hash),
        ProtoCodecImpl::encode(header.next_validators_hash),
        ProtoCodecImpl::encode(header.consensus_hash),
        ProtoCodecImpl::encode(header.app_hash),
        ProtoCodecImpl::encode(header.last_results_hash),
        ProtoCodecImpl::encode(header.evidence_hash),
        ProtoCodecImpl::encode(header.proposer_address),
    ];

    let hash_bytes = MerkleHashImpl::hash_byte_vectors(header_bytes.span());

    assert(commit_hash == @u32_8_to_array_u8(hash_bytes), CometErrors::INVALID_COMMIT_HASH);
}

pub fn valid_commit(signed_header: @SignedHeader, validators: @ValidatorSet) {
    valid_commit_validate(signed_header, validators);
    valid_commit_validate_full(signed_header, validators);
}

pub fn valid_commit_validate(signed_header: @SignedHeader, validators: @ValidatorSet) {
    assert(
        signed_header.commit.signatures.len() == validators.validators.len(),
        CometErrors::INVALID_SIGNATURE_COUNT,
    );

    let mut count_absent_votes = 0;

    let mut signatures = signed_header.commit.signatures.span();
    while let Some(elem) = signatures.pop_front() {
        let block_id_flag = elem.block_id_flag;
        if (block_id_flag != @BlockIdFlag::Commit && block_id_flag != @BlockIdFlag::Nil) {
            count_absent_votes += 1;
        }
    }

    // requires at least one non-absent vote
    assert(count_absent_votes < validators.validators.len(), CometErrors::INVALID_SIGNATURE_COUNT);
}

pub fn valid_commit_validate_full(signed_header: @SignedHeader, validators: @ValidatorSet) {
    let mut signatures = signed_header.commit.signatures.span();
    while let Some(elem) = signatures.pop_front() {
        if elem.block_id_flag == @BlockIdFlag::Unknown
            || elem.block_id_flag == @BlockIdFlag::Absent {
            continue;
        }

        let validator_address = elem.validator_address;

        let mut validators_span = validators.validators.span();

        let mut found = false;

        while let Some(elem) = validators_span.pop_front() {
            if elem.address == validator_address {
                found = true;
                break;
            }
        }

        assert(found, CometErrors::INVALID_SIGNATURE_LENGTH);
    }
}

pub fn is_within_trust_period(
    trusted_header_time: @Timestamp, trusting_period: @Duration, now: Timestamp,
) {
    let mut expires_at = trusted_header_time.clone();
    expires_at.seconds += *trusting_period.seconds;
    expires_at.nanos += *trusting_period.nanos;

    if expires_at.nanos >= 1_000_000_000 {
        let quo = expires_at.nanos / 1_000_000_000;
        let rem = expires_at.nanos % 1_000_000_000;

        expires_at.seconds += quo.into();
        expires_at.nanos = rem;
    }

    assert(now < expires_at, CometErrors::TRUSTED_HEADER_EXPIRED);
}

pub fn is_monotonic_bft_time(untrusted_time: @Timestamp, trusted_time: @Timestamp) {
    assert(untrusted_time > trusted_time, CometErrors::NON_MONOTONIC_BFT_TIME);
}

pub fn is_matching_chain_id(untrusted_chain_id: @ByteArray, trusted_chain_id: @ByteArray) {
    assert(untrusted_chain_id == trusted_chain_id, CometErrors::CHAIN_ID_MISMATCH);
}

pub fn valid_next_validator_set(
    untrusted_validator_hash: @Array<u8>, trusted_next_validator_hash: @Array<u8>,
) {
    assert(
        untrusted_validator_hash == trusted_next_validator_hash,
        CometErrors::INVALID_NEXT_VALIDATOR_SET,
    );
}

pub fn is_monotonic_height(untrusted_height: @u64, trusted_height: @u64) {
    assert(untrusted_height > trusted_height, CometErrors::NON_MONOTONIC_HEIGHT);
}

pub fn is_header_from_past(
    untrusted_header_time: @Timestamp, clock_drift: @Duration, now: Timestamp,
) {
    let mut drifted = now.clone();

    drifted.seconds += *clock_drift.seconds;
    drifted.nanos += *clock_drift.nanos;

    if drifted.nanos >= 1_000_000_000 {
        let quo = drifted.nanos / 1_000_000_000;
        let rem = drifted.nanos % 1_000_000_000;

        drifted.seconds += quo.into();
        drifted.nanos = rem;
    }

    assert(untrusted_header_time < @drifted, CometErrors::NON_MONOTONIC_BFT_TIME);
}


/// Verify that a) there is enough overlap between the validator sets of the
/// trusted and untrusted blocks and b) more than 2/3 of the validators
/// correctly committed the block.
pub fn verify_commit_against_trusted(
    untrusted: UntrustedBlockState, trusted: TrustedBlockState, options: Options,
) {
    let trusted_next_height = trusted.height.checked_add(1);
    assert(trusted_next_height.is_some(), CometErrors::OVERFLOWED_BLOCK_HEIGHT);
    if untrusted.height() == @trusted_next_height.unwrap() {
        check_signers_overlap(untrusted.signed_header.clone(), untrusted.validators.clone());
    } else {
        check_enough_trust_and_signers(
            untrusted.signed_header.clone(),
            untrusted.validators.clone(),
            trusted.next_validators,
            options.trust_threshold,
        );
    }
}

pub fn check_enough_trust_and_signers(
    untrusted_sh: SignedHeader,
    untrusted_validators: ValidatorSet,
    trusted_validators: ValidatorSet,
    trust_threshold: Fraction,
) {
    let (trusted_power, untrusted_power) = voting_power_in_sets(
        untrusted_sh, trusted_validators, trust_threshold, untrusted_validators, TWO_THIRDS,
    );
    assert(trusted_power.has_enough_power(), CometErrors::INSUFFICIENT_VOTING_POWER);
    assert(untrusted_power.has_enough_power(), CometErrors::INSUFFICIENT_VOTING_POWER);
}

pub fn check_signers_overlap(untrusted_header: SignedHeader, untrusted_validators: ValidatorSet) {
    let power = voting_power_in(untrusted_header, untrusted_validators, TWO_THIRDS);
    assert(power.has_enough_power(), CometErrors::INSUFFICIENT_VOTING_POWER);
}

fn voting_power_in(
    signed_header: SignedHeader, validator_set: ValidatorSet, trust_threshold: Fraction,
) -> VotingPowerTally {
    let mut votes = NonAbsentCommitVotesTrait::new(signed_header);
    voting_power_in_impl(
        ref votes, validator_set.clone(), trust_threshold, validator_set.total_power(),
    )
}

fn voting_power_in_sets(
    signed_header: SignedHeader,
    first_validator_set: ValidatorSet,
    first_trust_threshold: Fraction,
    second_validator_set: ValidatorSet,
    second_trust_threshold: Fraction,
) -> (VotingPowerTally, VotingPowerTally) {
    let mut votes = NonAbsentCommitVotesTrait::new(signed_header);
    let first_tally = voting_power_in_impl(
        ref votes,
        first_validator_set.clone(),
        first_trust_threshold,
        first_validator_set.total_power(),
    );
    let second_tally = voting_power_in_impl(
        ref votes,
        second_validator_set.clone(),
        second_trust_threshold,
        second_validator_set.total_power(),
    );
    (first_tally, second_tally)
}

fn voting_power_in_impl(
    ref votes: NonAbsentCommitVotes,
    validator_set: ValidatorSet,
    trust_threshold: Fraction,
    total_voting_power: u64,
) -> VotingPowerTally {
    let mut power = VotingPowerTallyTrait::new(total_voting_power, trust_threshold);
    for validator in validator_set.validators {
        if votes.has_voted(@validator) {
            power.tally(validator.voting_power);
            if power.clone().has_enough_power() {
                break;
            }
        }
    }
    power
}
