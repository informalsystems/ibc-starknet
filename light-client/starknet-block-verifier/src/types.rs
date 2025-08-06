use core::str::FromStr;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use starknet_core::types::Felt;
use starknet_crypto_lib::StarknetCryptoFunctions;

pub const STARKNET_BLOCK_HASH0: &[u8] = b"STARKNET_BLOCK_HASH0";
pub const STARKNET_BLOCK_HASH1: &[u8] = b"STARKNET_BLOCK_HASH1";

pub const STARKNET_GAS_PRICES0: &[u8] = b"STARKNET_GAS_PRICES0";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    pub block_hash: Felt,
    pub signature: [Felt; 2],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum L1DataAvailabilityMode {
    #[serde(alias = "Calldata")]
    Calldata,
    #[serde(alias = "Blob")]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

    pub l2_gas_price: Option<GasPrices>,

    pub parent_block_hash: Felt,

    pub block_hash: Felt,

    pub starknet_version: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StarknetVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

impl StarknetVersion {
    pub const fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self { major, minor, patch }
    }
}

impl FromStr for StarknetVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(3, '.');
        let major = iter.next().and_then(|s| s.parse().ok()).ok_or(())?;
        let minor = iter.next().and_then(|s| s.parse().ok()).ok_or(())?;
        let patch = iter.next().and_then(|s| s.parse().ok()).ok_or(())?;
        iter.next().is_none().then_some(()).ok_or(())?;
        Ok(Self { major, minor, patch })
    }
}


impl Block {
    pub fn hash_version(&self) -> &'static [u8] {
        let current_starknet_version = StarknetVersion::from_str(&self.starknet_version)
            .expect("Invalid Starknet version format");

        // https://github.com/starkware-libs/sequencer/blob/c16dbb0/crates/starknet_api/src/block_hash/block_hash_calculator.rs#L60

        if current_starknet_version < StarknetVersion::new(0, 13, 4) {
            STARKNET_BLOCK_HASH0
        } else {
            STARKNET_BLOCK_HASH1
        }
    }

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
    pub fn gas_commitment<C: StarknetCryptoFunctions>(&self, crypto_lib: &C) -> Vec<Felt> {
        if self.hash_version() == STARKNET_BLOCK_HASH0 {
            vec![
                self.l1_gas_price.price_in_wei,
                self.l1_gas_price.price_in_fri,
                self.l1_data_gas_price.price_in_wei,
                self.l1_data_gas_price.price_in_fri,
            ]
        } else if self.hash_version() == STARKNET_BLOCK_HASH1 {
            let l2_gas_price = self
                .l2_gas_price
                .as_ref()
                .expect("expected L2 gas price to be present");

            vec![crypto_lib.poseidon_hash_many(&[
                Felt::from_bytes_be_slice(STARKNET_GAS_PRICES0),
                self.l1_gas_price.price_in_wei,
                self.l1_gas_price.price_in_fri,
                self.l1_data_gas_price.price_in_wei,
                self.l1_data_gas_price.price_in_fri,
                l2_gas_price.price_in_wei,
                l2_gas_price.price_in_fri,
            ])]
        } else {
            unreachable!()
        }
    }

    /// Computes the Starknet 0.13.5 block hash.
    ///
    /// https://github.com/starkware-libs/sequencer/blob/c16dbb0/crates/starknet_api/src/block_hash/block_hash_calculator.rs#L111-L116
    pub fn compute_hash<C: StarknetCryptoFunctions>(&self, crypto_lib: &C) -> Felt {
        let mut elems = vec![];

        elems.extend_from_slice(&[
            Felt::from_bytes_be_slice(self.hash_version()),
            self.block_number.into(),
            self.state_root,
            self.sequencer_address,
            self.timestamp.into(),
            self.concatenated_counts(),
            self.state_diff_commitment.unwrap_or(Felt::ZERO),
            self.transaction_commitment,
            self.event_commitment,
            self.receipt_commitment.unwrap_or(Felt::ZERO),
        ]);

        elems.extend_from_slice(&self.gas_commitment(crypto_lib));

        elems.extend_from_slice(&[
            Felt::from_bytes_be_slice(self.starknet_version.as_bytes()),
            Felt::ZERO,
            self.parent_block_hash,
        ]);

        crypto_lib.poseidon_hash_many(&elems)
    }

    pub fn validate<C: StarknetCryptoFunctions>(&self, crypto_lib: &C) -> bool {
        self.block_hash == self.compute_hash(crypto_lib)
    }

    pub fn verify_signature<C: StarknetCryptoFunctions>(
        &self,
        crypto_lib: &C,
        signature: &Signature,
        public_key: &Felt,
    ) -> Result<bool, C::Error> {
        Ok(self.validate(crypto_lib)
            && signature.block_hash == self.block_hash
            && crypto_lib.verify(
                public_key,
                &signature.block_hash,
                &signature.signature[0],
                &signature.signature[1],
            )?)
    }
}
