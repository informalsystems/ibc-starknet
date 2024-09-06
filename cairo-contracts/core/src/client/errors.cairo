pub mod ClientErrors {
    pub const ZERO_CLIENT_TYPE: felt252 = 'ICS02: client type is 0';
    pub const ZERO_CLIENT_ADDRESS: felt252 = 'ICS02: client address is 0';
    pub const EMPTY_CLIENT_STATE: felt252 = 'ICS02: empty client state';
    pub const EMPTY_CONSENSUS_STATE: felt252 = 'ICS02: empty consensus state';
    pub const EMPTY_CLIENT_MESSAGE: felt252 = 'ICS02: empty client message';
    pub const INVALID_SUBSTITUTE_CLIENT_ID: felt252 = 'ICS02: invalid subs client id';
    pub const OVERFLOWED_HEIGHT: felt252 = 'ICS02: overflowed height';
}
