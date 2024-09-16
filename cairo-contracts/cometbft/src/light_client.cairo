use cometbft::ibc::{Height, MerkleRoot};
use cometbft::ics23::ProofSpec;
use protobuf::types::wkt::{Timestamp, Duration};
use cometbft::utils::Fraction;
use cometbft::types::{SignedHeader, ValidatorSet};

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct ClientState {
    chain_id: ByteArray,
    trust_level: Fraction,
    trusting_period: Duration,
    unbonding_period: Duration,
    max_clock_drift: Duration,
    frozen_height: Height,
    latest_height: Height,
    proof_specs: Array<ProofSpec>,
    upgrade_path: Array<ByteArray>,
    allow_update_after_expiry: bool,
    allow_update_after_misbehaviour: bool,
}


#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct ConsensusState {
    timestamp: Timestamp,
    root: MerkleRoot,
    next_validators_hash: ByteArray,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Misbehaviour {
    client_id: ByteArray,
    header_1: Header,
    header_2: Header,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Header {
    signed_header: SignedHeader,
    validator_set: ValidatorSet,
    trusted_height: Height,
    trusted_validator_set: ValidatorSet,
}
