pub mod ClientErrors {
    pub const UNSUPPORTED_CLIENT_TYPE: felt252 = 'ICS02: unsupported client type';
    pub const ZERO_CLIENT_TYPE: felt252 = 'ICS02: zero client type';
    pub const EMPTY_CLIENT_STATE: felt252 = 'ICS02: empty client state';
    pub const EMPTY_CONSENSUS_STATE: felt252 = 'ICS02: empty consensus state';
    pub const EMPTY_CLIENT_MESSAGE: felt252 = 'ICS02: empty client message';
    pub const INVALID_SUBSTITUTE_CLIENT_ID: felt252 = 'ICS02: invalid subs client id';
    pub const OVERFLOWED_HEIGHT: felt252 = 'ICS02: overflowed height';
}
