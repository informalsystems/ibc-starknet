use cgp_core::prelude::*;

#[derive_component(BlobTypeComponent, ProvideBlobType<Chain>)]
pub trait HasBlobType: Async {
    type Blob: Async;
}
