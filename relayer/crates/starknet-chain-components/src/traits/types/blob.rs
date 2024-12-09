use cgp::prelude::*;

#[cgp_component {
  name: BlobTypeComponent,
  provider: ProvideBlobType,
  context: Chain,
}]
pub trait HasBlobType: Async {
    type Blob: Async;
}
