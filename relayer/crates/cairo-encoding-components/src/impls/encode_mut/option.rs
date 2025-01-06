use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};

use crate::impls::encode_mut::variant_from::EncodeVariantFrom;

pub struct EncodeOption<T>(pub PhantomData<T>);

delegate_components! {
    <T> EncodeOption<T> {
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]: EncodeVariantFrom<TransformOption<T>>,
    }
}

pub struct TransformOption<T>(pub PhantomData<T>);

impl<T> TransformerRef for TransformOption<T> {
    type From = Option<T>;
    type To<'a>
        = Sum![&'a T, ()]
    where
        Self: 'a;

    fn transform<'a>(value: &'a Option<T>) -> Sum![&'a T, ()] {
        match value {
            Some(value) => Either::Left(value),
            None => Either::Right(Either::Left(())),
        }
    }
}

impl<T> Transformer for TransformOption<T> {
    type From = Sum![T, ()];

    type To = Option<T>;

    fn transform(value: Sum![T, ()]) -> Option<T> {
        match value {
            Either::Left(value) => Some(value),
            Either::Right(Either::Left(())) => None,
            Either::Right(Either::Right(value)) => match value {},
        }
    }
}
