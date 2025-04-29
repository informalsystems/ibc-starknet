use hermes_prelude::*;

#[cgp_type]
pub trait HasBlobType: Async {
    type Blob: Async;
}
