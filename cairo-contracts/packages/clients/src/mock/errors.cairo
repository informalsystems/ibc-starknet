pub mod MockErrors {
    pub const INACTIVE_CLIENT: felt252 = 'ICS07: inactive client';
    pub const ACTIVE_CLIENT: felt252 = 'ICS07: active client';
    pub const INVALID_CLIENT_TYPE: felt252 = 'ICS07: invalid client type';
    pub const INVALID_CLIENT_STATE: felt252 = 'ICS07: invalid client state';
    pub const INVALID_CONSENSUS_STATE: felt252 = 'ICS07: invalid consensus state';
    pub const INVALID_HEADER: felt252 = 'ICS07: invalid header';
    pub const INVALID_HEADER_FROM_FUTURE: felt252 = 'ICS07: inv header from future';
    pub const INVALID_CLIENT_SUBSTITUTE: felt252 = 'ICS07: invalid substitute';
    pub const MISSING_CLIENT_STATE: felt252 = 'ICS07: missing client state';
    pub const MISSING_CONSENSUS_STATE: felt252 = 'ICS07: missing consensus state';
    pub const MISSING_CLIENT_PROCESSED_TIME: felt252 = 'ICS07: missing processed time';
    pub const MISSING_CLIENT_PROCESSED_HEIGHT: felt252 = 'ICS07: missing processed height';
    pub const ZERO_UPDATE_HEIGHTS: felt252 = 'ICS07: zero update heights';
    pub const INVALID_UPGRADE_HEIGHT: felt252 = 'ICS07: invalid upgrade height';
    pub const INVALID_UPGRADE_PATH_LENGTH: felt252 = 'ICS07: invalid upgrade path len';
}
