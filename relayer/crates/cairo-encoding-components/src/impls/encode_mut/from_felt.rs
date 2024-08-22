use hermes_encoding_components::traits::encode_mut::{CanEncodeMut, MutEncoder};
use starknet::core::types::Felt;

pub struct EncodeFromFelt;

impl<Strategy, Encoding, Value> MutEncoder<Encoding, Strategy, Value> for EncodeFromFelt
where
    Encoding: CanEncodeMut<Strategy, Felt>,
    Value: Clone + Into<Felt>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.clone().into(), buffer)
    }
}
