pub mod component;
pub mod errors;
pub mod interface;
pub mod types;

/// The poseidon hash of the transfer port id.
pub const TRANSFER_PORT_ID_HASH: felt252 = 506076466176013583354797631368330115868609515147080483618120063858966368900;

pub fn TRANSFER_PORT_ID() -> ByteArray {
    "transfer"
}

