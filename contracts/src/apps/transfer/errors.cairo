pub mod ICS20Errors {
    pub const ALREADY_LISTED_TOKEN: felt252 = 'ICS20: token is already listed';
    pub const ZERO_TOKEN_ADDRESS: felt252 = 'ICS20: token address is 0';
    pub const UNAUTHORIZED_REGISTAR: felt252 = 'ICS20: unauthorized registrar';
    pub const MAXIMUM_MEMO_LENGTH: felt252 = 'ICS20: memo exceeds max length';
    pub const INSUFFICIENT_BALANCE: felt252 = 'ICS20: insufficient balance';
}
