use core::num::traits::{CheckedAdd, Zero};
use core::traits::PartialOrd;
use ics23::IntoArrayU32;
use starknet::SyscallResult;
use starknet::storage_access::{StorageBaseAddress, Store};
use starknet_ibc_core::client::ClientErrors;
use starknet_ibc_core::commitment::U32CollectorImpl;
use starknet_ibc_core::host::ClientId;

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
            Status::Frozen => true,
            _ => false,
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

pub impl HeightIntoArrayU32 of IntoArrayU32<Height> {
    fn into_array_u32(self: Height) -> (Array<u32>, u32, u32) {
        let mut coll = U32CollectorImpl::init();
        coll.extend(self.revision_number);
        coll.extend(self.revision_height);
        (coll.value(), 0, 0)
    }
}

pub impl StoreHeightArray of Store<Array<Height>> {
    fn read(address_domain: u32, base: StorageBaseAddress) -> SyscallResult<Array<Height>> {
        Self::read_at_offset(address_domain, base, 0)
    }

    fn write(
        address_domain: u32, base: StorageBaseAddress, value: Array<Height>,
    ) -> SyscallResult<()> {
        Self::write_at_offset(address_domain, base, 0, value)
    }

    fn read_at_offset(
        address_domain: u32, base: StorageBaseAddress, mut offset: u8,
    ) -> SyscallResult<Array<Height>> {
        let mut arr: Array<Height> = array![];

        let len: u8 = Store::<u8>::read_at_offset(address_domain, base, offset)
            .expect('Storage Span too large');
        offset += 1;

        let exit = 2 * len + offset;
        loop {
            if offset >= exit {
                break;
            }

            let value = Store::<Height>::read_at_offset(address_domain, base, offset).unwrap();
            arr.append(value);
            offset += Store::<Height>::size();
        };

        Result::Ok(arr)
    }

    fn write_at_offset(
        address_domain: u32, base: StorageBaseAddress, mut offset: u8, mut value: Array<Height>,
    ) -> SyscallResult<()> {
        let len: u8 = value.len().try_into().expect('Storage - Span too large');
        Store::<u8>::write_at_offset(address_domain, base, offset, len).unwrap();
        offset += 1;

        while let Option::Some(element) = value.pop_front() {
            Store::<Height>::write_at_offset(address_domain, base, offset, element).unwrap();
            offset += Store::<Height>::size();
        };

        Result::Ok(())
    }

    fn size() -> u8 {
        100 * Store::<Height>::size()
    }
}

#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct Timestamp {
    pub timestamp: u64,
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

pub impl TimestampIntoArrayU32 of IntoArrayU32<Timestamp> {
    fn into_array_u32(self: Timestamp) -> (Array<u32>, u32, u32) {
        self.timestamp.into_array_u32()
    }
}

