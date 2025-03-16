pub mod CometErrors {
    pub const INVALID_SIGNATURE_LENGTH: felt252 = 'ICS07: invalid signature length';
    pub const INVALID_PUBKEY_LENGTH: felt252 = 'ICS07: invalid pubkey length';
    pub const INVALID_ED25519_SIGNATURE: felt252 = 'ICS07: invalid ed25519 sig';
    pub const UNSUPPORTED_PUBKEY_TYPE: felt252 = 'ICS07: unsupported pubkey type';
}
