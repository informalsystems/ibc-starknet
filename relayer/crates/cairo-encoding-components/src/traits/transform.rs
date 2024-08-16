pub trait Transformer<From> {
    type To;

    fn transform(from: From) -> Self::To;
}
