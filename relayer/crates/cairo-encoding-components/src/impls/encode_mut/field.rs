use std::marker::PhantomData;

use cgp_core::field::HasField;
use hermes_encoding_components::traits::encode_mut::{CanEncodeMut, MutEncoder};

pub struct EncodeField<Tag>(pub PhantomData<Tag>);

impl<Encoding, Strategy, Context, Tag> MutEncoder<Encoding, Strategy, Context> for EncodeField<Tag>
where
    Encoding: CanEncodeMut<Strategy, Context::Field>,
    Context: HasField<Tag>,
{
    fn encode_mut(
        encoding: &Encoding,
        context: &Context,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let value = context.get_field(PhantomData);

        encoding.encode_mut(value, buffer)
    }
}
