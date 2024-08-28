use std::marker::PhantomData;

pub struct Tagged<Tag, Value> {
    pub value: Value,
    pub phantom: PhantomData<Tag>,
}

impl<Tag, Value> From<Value> for Tagged<Tag, Value> {
    fn from(value: Value) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}
