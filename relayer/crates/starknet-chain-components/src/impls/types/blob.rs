use cgp::prelude::*;
use starknet::core::types::Felt;

use crate::traits::types::blob::{BlobTypeComponent, ProvideBlobType};

pub struct ProvideFeltBlobType;

#[cgp_provider(BlobTypeComponent)]
impl<Chain: Async> ProvideBlobType<Chain> for ProvideFeltBlobType {
    type Blob = Vec<Felt>;
}
