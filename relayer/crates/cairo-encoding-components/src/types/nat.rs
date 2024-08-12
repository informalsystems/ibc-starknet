use core::marker::PhantomData;

pub struct Z;

pub struct S<N>(pub PhantomData<N>);

pub trait Nat {
    const N: usize;
}

impl Nat for Z {
    const N: usize = 0;
}

impl<N: Nat> Nat for S<N> {
    const N: usize = N::N + 1;
}
