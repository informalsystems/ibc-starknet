use crate::types::either::{Either, Void};
use crate::types::nat::{Nat, S, Z};

pub trait HasSize {
    type Size: Nat;
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
