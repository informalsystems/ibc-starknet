pub mod ICS20Errors {
    pub const NO_SEND_CAPABILITY: felt252 = 'ICS20: No send capability';
    pub const NO_RECEIVE_CAPABILITY: felt252 = 'ICS20: No receive capability';
    pub const INVALID_SENDER: felt252 = 'ICS20: Invalid sender account';
    pub const INVALID_RECEIVEER: felt252 = 'ICS20: Invalid receiver account';
    pub const ZERO_AMOUNT: felt252 = 'ICS20: transfer amount is 0';
    pub const INVALID_TOKEN_ADDRESS: felt252 = 'ICS20: invalid token address';
    pub const INVALID_DENOM: felt252 = 'ICS20: invalid denom';
    pub const MAXIMUM_MEMO_LENGTH: felt252 = 'ICS20: memo exceeds max length';
    pub const INSUFFICIENT_BALANCE: felt252 = 'ICS20: insufficient balance';
}
