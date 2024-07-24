pub mod ICS20Errors {
    pub const NO_SEND_CAPABILITY: felt252 = 'ICS20: No send capability';
    pub const NO_RECEIVE_CAPABILITY: felt252 = 'ICS20: No receive capability';
    pub const ZERO_TOKEN_NAME: felt252 = 'ICS20: token name is 0';
    pub const ZERO_TOKEN_ADDRESS: felt252 = 'ICS20: token address is 0';
    pub const ALREADY_LISTED_TOKEN: felt252 = 'ICS20: token is already listed';
    pub const INVALID_TOKEN_NAME: felt252 = 'ICS20: token name is invalid';
    pub const UNAUTHORIZED_REGISTAR: felt252 = 'ICS20: unauthorized registrar';
    pub const MAXIMUM_MEMO_LENGTH: felt252 = 'ICS20: memo exceeds max length';
    pub const INSUFFICIENT_BALANCE: felt252 = 'ICS20: insufficient balance';
}
