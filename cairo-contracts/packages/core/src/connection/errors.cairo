pub mod ConnectionErrors {
    pub const EMPTY_CONN_END_PROOF: felt252 = 'ICS03: empty conn end proof';
    pub const MISSING_CLIENT_ID: felt252 = 'ICS03: missing client ID';
    pub const MISSING_CONNECTION_END: felt252 = 'ICS03: missing connection end';
    pub const ZERO_CONNECTIONS: felt252 = 'ICS03: zero connections';
    pub const ZERO_PROOF_HEIGHT: felt252 = 'ICS03: zero proof height';
    pub const UNSUPPORTED_VERSION: felt252 = 'ICS03: unsupported version';
}
