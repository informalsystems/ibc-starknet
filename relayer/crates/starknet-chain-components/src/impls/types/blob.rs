use cgp::core::Async;
use starknet::core::types::Felt;

use crate::traits::types::blob::ProvideBlobType;

pub struct ProvideFeltBlobType;

impl<Chain: Async> ProvideBlobType<Chain> for ProvideFeltBlobType {
    type Blob = Vec<Felt>;
}
