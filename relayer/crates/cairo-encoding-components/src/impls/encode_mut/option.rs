use std::marker::PhantomData;

use cgp_core::prelude::*;

use crate::impls::encode_mut::variant_from::EncodeVariantFrom;
use crate::traits::encode_mut::MutEncoderComponent;
use crate::traits::transform::TransformerRef;
use crate::types::either::Either;
use crate::types::nat::{S, Z};
use crate::Sum;

pub struct EncodeOption<T>(pub PhantomData<T>);

delegate_components! {
    <T> EncodeOption<T> {
        MutEncoderComponent: EncodeVariantFrom<S<Z>, TransformOption<T>>
    }
}

pub struct TransformOption<T>(pub PhantomData<T>);

impl<T> TransformerRef for TransformOption<T> {
    type From = Option<T>;
    type To<'a> = Sum![&'a T, ()]
        where Self: 'a
    ;

    fn transform<'a>(value: &'a Option<T>) -> Sum![&'a T, ()] {
        match value {
            Some(value) => Either::Left(value),
            None => Either::Right(Either::Left(())),
        }
    }
}
