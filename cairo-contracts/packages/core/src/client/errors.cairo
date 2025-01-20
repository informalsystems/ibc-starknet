pub mod ClientErrors {
    pub const ZERO_CLIENT_TYPE: felt252 = 'ICS02: client type is 0';
    pub const ZERO_CLIENT_ADDRESS: felt252 = 'ICS02: client address is 0';
    pub const ZERO_RELAYER_ADDRESS: felt252 = 'ICS02: relayer address is 0';
    pub const EMPTY_CLIENT_STATE: felt252 = 'ICS02: empty client state';
    pub const EMPTY_CONSENSUS_STATE: felt252 = 'ICS02: empty consensus state';
    pub const EMPTY_CLIENT_MESSAGE: felt252 = 'ICS02: empty client message';
    pub const INACTIVE_CLIENT: felt252 = 'ICS02: inactive client';
    pub const INVALID_PROOF_HEIGHT: felt252 = 'ICS04: invalid proof height';
    pub const INVALID_GOVERNOR: felt252 = 'ICS02: invalid governor';
    pub const INVALID_SUBSTITUTE_CLIENT_ID: felt252 = 'ICS02: invalid subs client id';
    pub const OVERFLOWED_HEIGHT: felt252 = 'ICS02: overflowed height';
    pub const OVERFLOWED_TIMESTAMP: felt252 = 'ICS02: overflowed timestamp';
    pub const RELAYER_ALREADY_REGISTERED: felt252 = 'ICS02: rly already registered';
    pub const UNAUTHORIZED_RELAYER: felt252 = 'ICS02: unauthorized relayer';
}
