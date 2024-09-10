pub mod ChannelErrors {
    pub const EMPTY_COMMITMENT_PROOF: felt252 = 'ICS04: empty commitment proof';
    pub const INVALID_CHANNEL_STATE: felt252 = 'ICS04: invalid channel state';
    pub const INVALID_COUNTERPARTY: felt252 = 'ICS04: invalid counterparty';
    pub const TIMED_OUT_PACKET: felt252 = 'ICS04: packet timed out';
    pub const INACTIVE_CLIENT: felt252 = 'ICS04: inactive client';
    pub const INVALID_PROOF_HEIGHT: felt252 = 'ICS04: invalid proof height';
}
