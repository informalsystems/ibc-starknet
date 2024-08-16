pub trait Transformer<From, To> {
    fn transform(from: From) -> To;
}
