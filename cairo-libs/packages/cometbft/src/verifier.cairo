use cometbft::errors::CometErrors;
use cometbft::types::{
    NonAbsentCommitVotes, NonAbsentCommitVotesTrait, Options, SignedHeader, TrustedBlockState,
    UntrustedBlockState, UntrustedBlockStateTrait, ValidatorSet, ValidatorSetTrait,
    VotingPowerTally, VotingPowerTallyTrait,
};
use cometbft::utils::{Fraction, TWO_THIRDS};
use core::num::traits::CheckedAdd;
use protobuf::types::wkt::Timestamp;

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

pub fn verify_validator_sets(untrusted: @UntrustedBlockState) {}

pub fn validate_against_trusted(
    untrusted: @UntrustedBlockState, trusted: @TrustedBlockState, options: @Options, now: Timestamp,
) {}

pub fn verify_header_is_from_past(
    untrusted: @UntrustedBlockState, options: @Options, now: Timestamp,
) {}

/// Verify that a) there is enough overlap between the validator sets of the
/// trusted and untrusted blocks and b) more than 2/3 of the validators
/// correctly committed the block.
pub fn verify_commit_against_trusted(
    untrusted: UntrustedBlockState, trusted: TrustedBlockState, options: Options,
) {
    let trusted_next_height = trusted.height.checked_add(1);
    assert(trusted_next_height.is_some(), CometErrors::OVERFLOWED_BLOCK_HEIGHT);
    if untrusted.height() != @trusted_next_height.unwrap() {
        check_enough_trust_and_signers(
            untrusted.signed_header.clone(),
            untrusted.validators.clone(),
            trusted.next_validators,
            options.trust_threshold,
        );
    } else {
        check_signers_overlap(untrusted.signed_header.clone(), untrusted.validators.clone());
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
