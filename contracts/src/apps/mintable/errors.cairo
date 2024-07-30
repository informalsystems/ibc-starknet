pub mod MintableErrors {
    pub const UNAUTHORIZED_MINTER: felt252 = 'Unauthorized minter';
    pub const UNAUTHORIZED_BURNER: felt252 = 'Unauthorized burner';
    pub const BURN_FROM_ZERO: felt252 = 'ERC20: burn from 0';
    pub const MINT_TO_ZERO: felt252 = 'ERC20: mint to 0';
    pub const INSUFFICIENT_BALANCE: felt252 = 'ERC20: insufficient balance';
    pub const INSUFFICIENT_SUPPLY: felt252 = 'ERC20: insufficient supply';
}

