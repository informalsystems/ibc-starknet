use starknet::core::types::Felt;

pub struct StarknetWallet {
    pub account_address: Felt,
    pub signing_key: Felt,
    pub public_key: Felt,
}
