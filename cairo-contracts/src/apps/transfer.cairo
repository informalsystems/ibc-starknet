pub mod component;
pub mod errors;
pub mod interface;
pub mod types;

pub const TRANSFER_PORT_ID_KEY: felt252 = 0;

pub fn TRANSFER_PORT_ID() -> ByteArray {
    "transfer"
}

