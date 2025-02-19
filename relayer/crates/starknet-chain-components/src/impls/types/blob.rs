use cgp::prelude::*;
use starknet::core::types::Felt;

use crate::components::chain::BlobTypeComponent;
use crate::traits::types::blob::ProvideBlobType;

pub struct ProvideFeltBlobType;

#[cgp_provider(BlobTypeComponent)]
impl<Chain: Async> ProvideBlobType<Chain> for ProvideFeltBlobType {
    type Blob = Vec<Felt>;
}
