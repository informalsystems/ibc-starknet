#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct Timestamp {
    pub timestamp: u64,
}
