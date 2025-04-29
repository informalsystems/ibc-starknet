use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;
use starknet::signers::SigningKey;

use crate::impls::StarknetAddress;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarknetWallet {
    pub account_address: StarknetAddress,
    pub signing_key: Felt,
    pub public_key: Felt,
}

impl StarknetWallet {
    pub fn from_signing_key(account_address: Felt, signing_key: Felt) -> Self {
        let public_key = SigningKey::from_secret_scalar(signing_key)
            .verifying_key()
            .scalar();

        Self {
            account_address: account_address.into(),
            signing_key,
            public_key,
        }
    }
}
