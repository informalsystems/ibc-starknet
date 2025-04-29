use derive_more::{Constructor, Deref, Display, From, FromStr};
use hermes_prelude::*;
use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Default,
    Constructor,
    Deref,
    Display,
    From,
    FromStr,
    Serialize,
    Deserialize,
    HasFields,
)]
#[display("0x{_0:x}")]
pub struct StarknetAddress(pub Felt);

#[cfg(test)]
mod tests {
    use starknet::core::types::Felt;

    use super::*;

    #[test]
    fn test_starknet_address_display() {
        let address = StarknetAddress(Felt::from(0x12345678));
        assert_eq!(format!("{address}"), "0x12345678");
    }
}
