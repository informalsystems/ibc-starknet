use protobuf::types::wkt::Timestamp;

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Consensus {
    pub block: u64,
    pub app: u64,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct PartSetHeader {
    pub total: u32,
    pub hash: ByteArray,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct BlockId {
    pub hash: ByteArray,
    pub part_set_header: PartSetHeader,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Header {
    pub version: Consensus,
    pub chain_id: ByteArray,
    pub height: i64,
    pub time: Timestamp,
    pub last_block_id: BlockId,
    pub last_commit_hash: ByteArray,
    pub data_hash: ByteArray,
    pub validators_hash: ByteArray,
    pub next_validators_hash: ByteArray,
    pub consensus_hash: ByteArray,
    pub app_hash: ByteArray,
    pub last_results_hash: ByteArray,
    pub evidence_hash: ByteArray,
    pub proposer_address: ByteArray,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub enum BlockIdFlag {
    #[default]
    Unknown,
    Absent,
    Commit,
    Nil,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct CommitSig {
    pub block_id_flag: BlockIdFlag,
    pub validator_address: ByteArray,
    pub timestamp: Timestamp,
    pub signature: ByteArray,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Commit {
    pub height: i64,
    pub round: i32,
    pub block_id: BlockId,
    pub signatures: Array<CommitSig>,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct SignedHeader {
    pub header: Header,
    pub commit: Commit,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct PublicKey {
    pub ed25519: ByteArray,
    pub secp256k1: ByteArray,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Validator {
    pub address: ByteArray,
    pub pub_key: PublicKey,
    pub voting_power: i64,
    pub proposer_priority: i64,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct ValidatorSet {
    pub validators: Array<Validator>,
    pub proposer: Validator,
    pub total_voting_power: i64,
}
