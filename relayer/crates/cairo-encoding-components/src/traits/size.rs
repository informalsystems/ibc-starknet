use cgp::prelude::{Either, Void};

use crate::types::nat::{S, Z};

pub trait HasSize {
    type Size;
}

impl<A, B> HasSize for Either<A, B>
where
    B: HasSize,
{
    type Size = S<B::Size>;
}

impl HasSize for Void {
    type Size = Z;
}
