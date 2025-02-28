use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;

use crate::impls::types::address::StarknetAddress;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SigningKey {
    File { file: String },
    Felt { felt: Felt },
}

impl TryFrom<SigningKey> for Felt {
    type Error = String;

    fn try_from(value: SigningKey) -> Result<Self, Self::Error> {
        match value {
            SigningKey::File { file } => Ok(std::fs::read_to_string(file)
                .map_err(|e| format!("{e}"))?
                .trim()
                .parse()
                .map_err(|e| format!("{e}"))?),
            SigningKey::Felt { felt } => Ok(felt),
        }
    }
}

impl From<Felt> for SigningKey {
    fn from(felt: Felt) -> Self {
        Self::Felt { felt }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarknetWallet {
    pub account_address: StarknetAddress,
    pub signing_key: SigningKey,
    pub public_key: Felt,
}
