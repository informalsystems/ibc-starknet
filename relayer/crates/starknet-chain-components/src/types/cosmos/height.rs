use hermes_prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, HasField, HasFields)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}
