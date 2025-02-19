pub mod CometErrors {
    pub const INACTIVE_CLIENT: felt252 = 'ICS07: inactive client';
    pub const INVALID_CLIENT_TYPE: felt252 = 'ICS07: invalid client type';
    pub const INVALID_CLIENT_STATE: felt252 = 'ICS07: invalid client state';
    pub const INVALID_CONSENSUS_STATE: felt252 = 'ICS07: invalid consensus state';
    pub const INVALID_HEADER: felt252 = 'ICS07: invalid header';
    pub const INVALID_HEADER_FROM_FUTURE: felt252 = 'ICS07: inv header from future';
    pub const INVALID_OWNER: felt252 = 'ICS07: invalid owner';
    pub const MISSING_CLIENT_STATE: felt252 = 'ICS07: missing client state';
    pub const MISSING_CONSENSUS_STATE: felt252 = 'ICS07: missing consensus state';
    pub const MISSING_CLIENT_PROCESSED_TIME: felt252 = 'ICS07: missing processed time';
    pub const MISSING_CLIENT_PROCESSED_HEIGHT: felt252 = 'ICS07: missing processed height';
    pub const ZERO_UPDATE_HEIGHTS: felt252 = 'ICS07: zero update heights';
    pub const ZERO_OWNER: felt252 = 'ICS07: zero owner';
}
