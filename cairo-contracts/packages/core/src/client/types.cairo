use cometbft::ibc::Height as ProtoHeight;
use core::num::traits::{CheckedAdd, OverflowingMul, Zero};
use core::traits::PartialOrd;
use ibc_utils::bytes::{IntoArrayU32, U64IntoArrayU32};
use protobuf::types::wkt::{Duration as ProtoDuration, Timestamp as ProtoTimestamp};
use starknet_ibc_core::client::ClientErrors;
use starknet_ibc_core::commitment::U32CollectorImpl;
use starknet_ibc_core::host::ClientId;

const NANOS_PER_SEC: u32 = 1_000_000_000;

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct CreateResponse {
    pub client_id: ClientId,
    pub height: Height,
}

#[generate_trait]
pub impl CreateResponseImpl of CreateResponseTrait {
    fn new(client_id: ClientId, height: Height) -> CreateResponse {
        CreateResponse { client_id, height }
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub enum UpdateResponse {
    Success: Array<Height>,
    Misbehaviour,
}

pub impl HeightsIntoUpdateResponse of Into<Array<Height>, UpdateResponse> {
    fn into(self: Array<Height>) -> UpdateResponse {
        UpdateResponse::Success(self)
    }
}

#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub enum Status {
    Active,
    // The default for cases when a client state isn't found during a storage call.
    #[default]
    Expired,
    Frozen: Height,
}

#[generate_trait]
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
            Status::Frozen(_) => true,
            _ => false,
        }
    }
}

pub impl StatusToFrozenHeight of Into<Status, Height> {
    fn into(self: Status) -> Height {
        match self {
            Status::Active => Height { revision_number: 0, revision_height: 0 },
            Status::Expired => Height { revision_number: 0, revision_height: 1 },
            Status::Frozen(height) => height,
        }
    }
}

#[derive(Copy, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}

#[generate_trait]
pub impl HeightImpl of HeightTrait {
    fn new(revision_number: u64, revision_height: u64) -> Height {
        Height { revision_number, revision_height }
    }
}

pub impl HeightZero of Zero<Height> {
    fn zero() -> Height {
        Height { revision_number: 0, revision_height: 0 }
    }

    fn is_zero(self: @Height) -> bool {
        self.revision_number == @0 && self.revision_height == @0
    }

    fn is_non_zero(self: @Height) -> bool {
        !self.is_zero()
    }
}

pub impl HeightAdd of Add<Height> {
    fn add(lhs: Height, rhs: Height) -> Height {
        let revision_number = lhs.revision_number.checked_add(rhs.revision_number);
        let revision_height = lhs.revision_height.checked_add(rhs.revision_height);

        assert(revision_number.is_some(), ClientErrors::OVERFLOWED_HEIGHT);
        assert(revision_height.is_some(), ClientErrors::OVERFLOWED_HEIGHT);

        Height {
            revision_number: revision_number.unwrap(), revision_height: revision_height.unwrap(),
        }
    }
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

pub impl HeightToProto of Into<Height, ProtoHeight> {
    fn into(self: Height) -> ProtoHeight {
        ProtoHeight { revision_number: self.revision_number, revision_height: self.revision_height }
    }
}

pub impl HeightIntoArrayU32 of IntoArrayU32<Height> {
    fn into_array_u32(self: Height) -> (Array<u32>, u32, u32) {
        let mut coll = U32CollectorImpl::init();
        coll.extend(self.revision_number);
        coll.extend(self.revision_height);
        (coll.value(), 0, 0)
    }
}

pub impl StoreHeightArray = ibc_utils::storage::StorePackingViaSerde<Array<Height>>;

/// Represents Unix timestamp in nanoseconds.
#[derive(Copy, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct Timestamp {
    pub timestamp: u64,
}

#[generate_trait]
pub impl TimestampImpl of TimestampTrait {
    /// Constructs a `Timestamp` given a Unix seconds.
    ///
    /// NOTE: `u64` can represent times up to about year 2554, thus normally it shouldn't
    /// overflow.
    fn from_unix_secs(seconds: u64) -> Timestamp {
        let (unix_nanos, overflowed) = seconds.overflowing_mul(NANOS_PER_SEC.into());
        assert(!overflowed, ClientErrors::OVERFLOWED_TIMESTAMP);
        Timestamp { timestamp: unix_nanos }
    }

    fn from_seconds_and_nanos(seconds: u64, nanos: u32) -> Timestamp {
        let (unix_nanos, overflowed) = seconds.overflowing_mul(NANOS_PER_SEC.into());
        assert(!overflowed, ClientErrors::OVERFLOWED_TIMESTAMP);
        let timestamp = unix_nanos.checked_add(nanos.into()).unwrap();
        Timestamp { timestamp }
    }

    /// Returns the timestamp in seconds, truncating the nanoseconds.
    fn as_secs(self: Timestamp) -> u64 {
        self.timestamp / NANOS_PER_SEC.into()
    }

    /// Returns the timestamp in nanoseconds.
    fn as_nanos(self: Timestamp) -> u64 {
        self.timestamp % NANOS_PER_SEC.into()
    }

    /// Returns the timestamp of the latest block in Starknet, which serves as the host timestamp in
    /// the IBC implementation.
    fn host() -> Timestamp {
        Self::from_unix_secs(starknet::get_block_timestamp())
    }
}

pub impl TimestampZero of Zero<Timestamp> {
    fn zero() -> Timestamp {
        Timestamp { timestamp: 0 }
    }

    fn is_zero(self: @Timestamp) -> bool {
        self.timestamp == @0
    }

    fn is_non_zero(self: @Timestamp) -> bool {
        !self.is_zero()
    }
}

pub impl TimestampAdd of Add<Timestamp> {
    fn add(lhs: Timestamp, rhs: Timestamp) -> Timestamp {
        let timestamp = lhs.timestamp.checked_add(rhs.timestamp);

        assert(timestamp.is_some(), ClientErrors::OVERFLOWED_TIMESTAMP);

        Timestamp { timestamp: timestamp.unwrap() }
    }
}

pub impl TimestampPartialOrd of PartialOrd<@Timestamp> {
    fn le(lhs: @Timestamp, rhs: @Timestamp) -> bool {
        lhs.timestamp <= rhs.timestamp
    }
    fn ge(lhs: @Timestamp, rhs: @Timestamp) -> bool {
        lhs.timestamp >= rhs.timestamp
    }
    fn lt(lhs: @Timestamp, rhs: @Timestamp) -> bool {
        lhs.timestamp < rhs.timestamp
    }
    fn gt(lhs: @Timestamp, rhs: @Timestamp) -> bool {
        lhs.timestamp > rhs.timestamp
    }
}

pub impl U64IntoTimestamp of Into<u64, Timestamp> {
    fn into(self: u64) -> Timestamp {
        Timestamp { timestamp: self }
    }
}

pub impl TimestampIntoU128 of Into<Timestamp, u128> {
    fn into(self: Timestamp) -> u128 {
        self.timestamp.into()
    }
}

pub impl TimestampIntoArrayU32 of IntoArrayU32<Timestamp> {
    fn into_array_u32(self: Timestamp) -> (Array<u32>, u32, u32) {
        self.timestamp.into_array_u32()
    }
}

pub impl TimestampToProto of TryInto<Timestamp, ProtoTimestamp> {
    fn try_into(self: Timestamp) -> Option<ProtoTimestamp> {
        let seconds = self.as_secs().try_into()?;
        let nanos = self.as_nanos().try_into()?;
        Some(ProtoTimestamp { seconds, nanos })
    }
}

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde, starknet::Store)]
pub struct Duration {
    pub seconds: u64,
    pub nanos: u32,
}

#[generate_trait]
pub impl DurationImpl of DurationTrait {
    fn as_nanos(self: Duration) -> u128 {
        self.seconds.into() * NANOS_PER_SEC.into() + self.nanos.into()
    }
}

pub impl DurationAdd of Add<Duration> {
    fn add(lhs: Duration, rhs: Duration) -> Duration {
        let seconds = lhs.seconds.checked_add(rhs.seconds);
        assert(seconds.is_some(), ClientErrors::OVERFLOWED_DURATION);
        let nanos = lhs.nanos.checked_add(rhs.nanos);
        assert(nanos.is_some(), ClientErrors::OVERFLOWED_DURATION);

        // nanos can overflow into seconds
        let (seconds, nanos) = if nanos.unwrap() >= NANOS_PER_SEC {
            let quotient = nanos.unwrap() / NANOS_PER_SEC;
            let remainder = nanos.unwrap() % NANOS_PER_SEC;
            (seconds.unwrap() + quotient.into(), remainder)
        } else {
            (seconds.unwrap(), nanos.unwrap())
        };

        Duration { seconds, nanos }
    }
}

impl DurationZero of Zero<Duration> {
    fn zero() -> Duration {
        Duration { seconds: 0, nanos: 0 }
    }
    fn is_zero(self: @Duration) -> bool {
        self == @Self::zero()
    }
    fn is_non_zero(self: @Duration) -> bool {
        !self.is_zero()
    }
}

pub impl DurationToProto of TryInto<Duration, ProtoDuration> {
    fn try_into(self: Duration) -> Option<ProtoDuration> {
        let seconds = self.seconds.try_into()?;
        let nanos = self.nanos.try_into()?;
        Some(ProtoDuration { seconds, nanos })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let timestamp = TimestampImpl::from_unix_secs(29);
        assert_eq!(timestamp.into(), 29_000_000_000_u128);
    }

    #[test]
    fn test_duration_simple_add() {
        let duration1 = Duration { seconds: 1, nanos: 1 };
        let duration2 = Duration { seconds: 2, nanos: 2 };
        let duration3 = Duration { seconds: 3, nanos: 3 };
        assert_eq!(duration1 + duration2, duration3);
    }

    #[test]
    fn test_duration_add_nano_overflow() {
        let duration1 = Duration { seconds: 1, nanos: 999_999_999 };
        let duration2 = Duration { seconds: 2, nanos: 999_999_999 };
        let duration3 = Duration { seconds: 4, nanos: 999_999_998 };
        assert_eq!(duration1 + duration2, duration3);
    }

    #[test]
    fn test_duration_as_nanos() {
        let duration = Duration { seconds: 1, nanos: 1 };
        assert_eq!(duration.as_nanos(), 1_000_000_001);
    }
}
