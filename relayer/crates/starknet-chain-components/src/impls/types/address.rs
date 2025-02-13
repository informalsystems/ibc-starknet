use cgp::core::Async;
use derive_more::{Deref, Display, From, FromStr};
use hermes_test_components::chain::traits::types::address::ProvideAddressType;
use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;
pub struct ProvideFeltAddressType;
use hermes_encoding_components::traits::decode_mut::{CanDecodeMut, MutDecoder};
use hermes_encoding_components::traits::encode_mut::{CanEncodeMut, MutEncoder};

impl<Chain: Async> ProvideAddressType<Chain> for ProvideFeltAddressType {
    type Address = StarknetAddress;
}

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
    Deref,
    Display,
    From,
    FromStr,
    Serialize,
    Deserialize,
)]
#[display("0x{_0:x}")]
pub struct StarknetAddress(Felt);

pub struct EncodeStarknetAddress;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, StarknetAddress> for EncodeStarknetAddress
where
    Encoding: CanEncodeMut<Strategy, Felt>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &StarknetAddress,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(value, buffer)?;
        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, StarknetAddress> for EncodeStarknetAddress
where
    Encoding: CanDecodeMut<Strategy, Felt>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<StarknetAddress, Encoding::Error> {
        let value = encoding.decode_mut(buffer)?;
        Ok(value.into())
    }
}

#[cfg(test)]
mod tests {
    use starknet::core::types::Felt;

    use super::*;

    #[test]
    fn test_starknet_address_display() {
        let address = StarknetAddress(Felt::from(0x12345678));
        assert_eq!(format!("{}", address), "0x12345678");
    }
}
