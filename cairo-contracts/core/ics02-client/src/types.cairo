use core::traits::PartialOrd;

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub enum UpdateResult {
    Success: Array<Height>,
    Misbehaviour,
}

pub impl HeightsIntoUpdateResult of Into<Array<Height>, UpdateResult> {
    fn into(self: Array<Height>) -> UpdateResult {
        UpdateResult::Success(self)
    }
}

#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub enum Status {
    Active,
    Expired,
    Frozen: Height,
}

pub trait StatusTrait {
    fn is_active(self: @Status) -> bool;
    fn is_expired(self: @Status) -> bool;
    fn is_frozen(self: @Status) -> bool;
}

pub impl StatusImpl of StatusTrait {
    fn is_active(self: @Status) -> bool {
        match self {
            Status::Active => true,
            _ => false,
        }
    }

    fn is_expired(self: @Status) -> bool {
        match self {
            Status::Expired => true,
            _ => false,
        }
    }

    fn is_frozen(self: @Status) -> bool {
        match self {
            Status::Frozen => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}

pub impl HeightPartialOrd of PartialOrd<Height> {
    fn le(lhs: Height, rhs: Height) -> bool {
        lhs.revision_number < rhs.revision_number
            || (lhs.revision_number == rhs.revision_number
                && lhs.revision_height <= rhs.revision_height)
    }
    fn ge(lhs: Height, rhs: Height) -> bool {
        lhs.revision_number > rhs.revision_number
            || (lhs.revision_number == rhs.revision_number
                && lhs.revision_height >= rhs.revision_height)
    }
    fn lt(lhs: Height, rhs: Height) -> bool {
        lhs.revision_number < rhs.revision_number
            || (lhs.revision_number == rhs.revision_number
                && lhs.revision_height < rhs.revision_height)
    }
    fn gt(lhs: Height, rhs: Height) -> bool {
        lhs.revision_number > rhs.revision_number
            || (lhs.revision_number == rhs.revision_number
                && lhs.revision_height > rhs.revision_height)
    }
}


#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct Timestamp {
    pub timestamp: u64,
}

