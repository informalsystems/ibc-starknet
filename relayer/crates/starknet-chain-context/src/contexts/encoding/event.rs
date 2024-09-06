use cgp::prelude::HasComponents;

pub struct StarknetEventEncoding;

pub struct StarknetEventEncodingComponents;

impl HasComponents for StarknetEventEncoding {
    type Components = StarknetEventEncodingComponents;
}
