pub mod ChannelErrors {
    pub const ACK_ALREADY_EXISTS: felt252 = 'ICS04: ack already exists';
    pub const EMPTY_COMMITMENT_PROOF: felt252 = 'ICS04: empty commitment proof';
    pub const INACTIVE_CLIENT: felt252 = 'ICS04: inactive client';
    pub const INVALID_CHANNEL_STATE: felt252 = 'ICS04: invalid channel state';
    pub const INVALID_COUNTERPARTY: felt252 = 'ICS04: invalid counterparty';
    pub const INVALID_PROOF_HEIGHT: felt252 = 'ICS04: invalid proof height';
    pub const INVALID_PACKET_SEQUENCE: felt252 = 'ICS04: invalid packet sequence';
    pub const MISSING_CHANNEL_END: felt252 = 'ICS04: missing channel end';
    pub const PACKET_ALREADY_RECEIVED: felt252 = 'ICS04: packet already received';
    pub const TIMED_OUT_PACKET: felt252 = 'ICS04: packet timed out';
    pub const UNSUPPORTED_PORT_ID: felt252 = 'ICS04: unsupported port id';
}
