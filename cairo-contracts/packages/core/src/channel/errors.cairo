pub mod ChannelErrors {
    pub const PACKET_RECEIPT_ALREADY_EXISTS: felt252 = 'ICS04: receipt already exists';
    pub const PACKET_ACK_ALREADY_EXISTS: felt252 = 'ICS04: ack already exists';
    pub const EMPTY_CHAN_END_PROOF: felt252 = 'ICS04: empty channel end proof';
    pub const EMPTY_COMMITMENT_PROOF: felt252 = 'ICS04: empty commitment proof';
    pub const EMPTY_ACK: felt252 = 'ICS04: empty acknowledgement';
    pub const EMPTY_UNRECEIVED_PROOF: felt252 = 'ICS04: empty unreceived proof';
    pub const EMPTY_ACK_PROOF: felt252 = 'ICS04: empty ack proof';
    pub const ZERO_PROOF_HEIGHT: felt252 = 'ICS04: zero proof height';
    pub const UNSUPPORTED_ORDERING: felt252 = 'ICS04: unsupported ordering';
    pub const INVALID_CHANNEL_STATE: felt252 = 'ICS04: invalid channel state';
    pub const INVALID_COUNTERPARTY: felt252 = 'ICS04: invalid counterparty';
    pub const INVALID_PACKET_SEQUENCE: felt252 = 'ICS04: invalid packet sequence';
    pub const MISSING_CONNECTION_ID: felt252 = 'ICS04: missing connection ID';
    pub const MISSING_CHANNEL_ID: felt252 = 'ICS04: missing channel ID';
    pub const MISSING_CHANNEL_END: felt252 = 'ICS04: missing channel end';
    pub const MISSING_CHANNEL_VERSION: felt252 = 'ICS04: missing channel version';
    pub const MISSING_PORT_ID: felt252 = 'ICS04: missing port ID';
    pub const MISSING_PACKET_RECEIPT: felt252 = 'ICS04: missing packet receipt';
    pub const MISSING_PACKET_COMMITMENT: felt252 = 'ICS04: missing commitment';
    pub const MISSING_PACKET_ACK: felt252 = 'ICS04: missing packet ack';
    pub const MISSING_PACKET_TIMEOUT: felt252 = 'ICS04: missing packet timeout';
    pub const MISMATCHED_PACKET_COMMITMENT: felt252 = 'ICS04: mismatched commitment';
    pub const MISMATCHED_PACKET_SEQUENCE: felt252 = 'ICS04: mismatched sequence';
    pub const PACKET_ALREADY_RECEIVED: felt252 = 'ICS04: packet already received';
    pub const PENDING_PACKET: felt252 = 'ICS04: packet not timed out';
    pub const TIMED_OUT_PACKET: felt252 = 'ICS04: packet timed out';
}
