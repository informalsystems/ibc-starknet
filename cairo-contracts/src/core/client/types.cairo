#[derive(Clone, Debug, Drop, PartialEq, Eq, Serde, Store)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}

#[derive(Clone, Debug, Drop, PartialEq, Eq, Serde, Store)]
pub struct Timestamp {
    pub timestamp: u64,
}
