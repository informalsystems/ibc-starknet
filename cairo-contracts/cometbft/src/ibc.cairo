#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct MerkleRoot {
    pub hash: ByteArray,
}
