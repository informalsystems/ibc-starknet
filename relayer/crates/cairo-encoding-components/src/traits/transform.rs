pub trait TransformerRef {
    type From;
    type To<'a>
    where
        Self: 'a;

    fn transform<'a>(from: &'a Self::From) -> Self::To<'a>;
}
