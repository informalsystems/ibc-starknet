use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use starknet_core::types::Felt;

use crate::StarknetCryptoFunctions;

pub const STARKNET_GAS_PRICES0: &[u8] = b"STARKNET_GAS_PRICES0";
pub const STARKNET_BLOCK_HASH1: &[u8] = b"STARKNET_BLOCK_HASH1";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    pub block_hash: Felt,
    pub signature: [Felt; 2],
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum L1DataAvailabilityMode {
    #[serde(alias = "Calldata")]
    Calldata,
    #[serde(alias = "Blob")]
    #[default]
    Blob,
}

impl From<L1DataAvailabilityMode> for u8 {
    fn from(mode: L1DataAvailabilityMode) -> Self {
        // https://github.com/starkware-libs/sequencer/blob/c16dbb0/crates/starknet_api/src/block_hash/block_hash_calculator.rs#L214-L217
        match mode {
            L1DataAvailabilityMode::Calldata => 0b0000_0000,
            L1DataAvailabilityMode::Blob => 0b1000_0000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GasPrices {
    pub price_in_wei: Felt,
    pub price_in_fri: Felt,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub block_number: u64,
    pub state_root: Felt,
    pub sequencer_address: Felt,
    pub timestamp: u64,

    pub transactions: Vec<Transaction>,
    pub transaction_receipts: Vec<TransactionReceipt>,

    pub state_diff_length: Option<u64>,
    pub l1_da_mode: L1DataAvailabilityMode,

    pub state_diff_commitment: Option<Felt>,
    pub transaction_commitment: Felt,
    pub event_commitment: Felt,
    pub receipt_commitment: Option<Felt>,

    pub l1_gas_price: GasPrices,
    pub l1_data_gas_price: GasPrices,
    pub l2_gas_price: GasPrices,

    pub parent_block_hash: Felt,

    pub block_hash: Felt,

    pub starknet_version: String,
}

impl Block {
    /// Computes the concatenated counts of the block.
    ///
    /// https://github.com/starkware-libs/sequencer/blob/c16dbb0/crates/starknet_api/src/block_hash/block_hash_calculator.rs#L204-L208
    pub fn concatenated_counts(&self) -> Felt {
        let l1_data_availability_byte: u8 = self.l1_da_mode.into();

        let concat_bytes = [
            (self.transactions.len() as u64).to_be_bytes(),
            (self
                .transaction_receipts
                .iter()
                .map(|receipt| receipt.events.len())
                .sum::<usize>() as u64)
                .to_be_bytes(),
            self.state_diff_length.unwrap_or_default().to_be_bytes(),
            [
                l1_data_availability_byte,
                0_u8,
                0_u8,
                0_u8,
                0_u8,
                0_u8,
                0_u8,
                0_u8,
            ],
        ]
        .concat();

        Felt::from_bytes_be_slice(concat_bytes.as_slice())
    }

    /// Computes the Starknet 0.13.5 gas commitment.
    ///
    /// https://github.com/starkware-libs/sequencer/blob/c16dbb0/crates/starknet_api/src/block_hash/block_hash_calculator.rs#L234-L242
    pub fn gas_commitment<C: StarknetCryptoFunctions>(&self) -> Felt {
        C::poseidon_hash_many(&[
            Felt::from_bytes_be_slice(STARKNET_GAS_PRICES0),
            self.l1_gas_price.price_in_wei,
            self.l1_gas_price.price_in_fri,
            self.l1_data_gas_price.price_in_wei,
            self.l1_data_gas_price.price_in_fri,
            self.l2_gas_price.price_in_wei,
            self.l2_gas_price.price_in_fri,
        ])
    }

    /// Computes the Starknet 0.13.5 block hash.
    ///
    /// https://github.com/starkware-libs/sequencer/blob/c16dbb0/crates/starknet_api/src/block_hash/block_hash_calculator.rs#L111-L116
    pub fn compute_hash<C: StarknetCryptoFunctions>(&self) -> Felt {
        C::poseidon_hash_many(&[
            Felt::from_bytes_be_slice(STARKNET_BLOCK_HASH1),
            self.block_number.into(),
            self.state_root,
            self.sequencer_address,
            self.timestamp.into(),
            self.concatenated_counts(),
            self.state_diff_commitment.unwrap_or(Felt::ZERO),
            self.transaction_commitment,
            self.event_commitment,
            self.receipt_commitment.unwrap_or(Felt::ZERO),
            self.gas_commitment::<C>(),
            Felt::from_bytes_be_slice(self.starknet_version.as_bytes()),
            Felt::ZERO,
            self.parent_block_hash,
        ])
    }

    pub fn validate<C: StarknetCryptoFunctions>(&self) -> bool {
        self.block_hash == self.compute_hash::<C>()
    }

    pub fn verify_signature<C: StarknetCryptoFunctions>(
        &self,
        signature: &Signature,
        public_key: &Felt,
    ) -> Result<bool, C::Error> {
        Ok(self.validate::<C>()
            && signature.block_hash == self.block_hash
            && C::verify(
                public_key,
                &signature.block_hash,
                &signature.signature[0],
                &signature.signature[1],
            )?)
    }
}
