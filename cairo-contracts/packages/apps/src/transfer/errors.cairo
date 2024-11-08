pub mod TransferErrors {
    pub const INVALID_SENDER: felt252 = 'ICS20: Invalid sender account';
    pub const INVALID_RECEIVER: felt252 = 'ICS20: Invalid receiver account';
    pub const ZERO_OWNER: felt252 = 'ICS20: owner is 0';
    pub const ZERO_ERC20_CLASS_HASH: felt252 = 'ICS20: erc20 class hash is 0';
    pub const ZERO_AMOUNT: felt252 = 'ICS20: transfer amount is 0';
    pub const ZERO_SALT: felt252 = 'ICS20: salt is 0';
    pub const ZERO_TOKEN_ADDRESS: felt252 = 'ICS20: missing token address';
    pub const INVALID_APP_VERSION: felt252 = 'ICS20: invalid app version';
    pub const INVALID_DENOM: felt252 = 'ICS20: invalid denom';
    pub const INVALID_PACKET_DATA: felt252 = 'ICS20: invalid packet data';
    pub const INVALID_OWNER: felt252 = 'ICS20: invalid owner';
    pub const EMPTY_ACK_STATUS: felt252 = 'ICS20: empty ack status';
    pub const MAXIMUM_MEMO_LENGTH: felt252 = 'ICS20: memo exceeds max length';
    pub const INSUFFICIENT_BALANCE: felt252 = 'ICS20: insufficient balance';
    pub const NO_SEND_CAPABILITY: felt252 = 'ICS20: No send capability';
    pub const NO_RECEIVE_CAPABILITY: felt252 = 'ICS20: No receive capability';
}
