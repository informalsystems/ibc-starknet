use cgp_core::prelude::*;

#[derive_component(AddressTypeComponent, ProvideAddressType<Chain>)]
pub trait HasAddressType: Async {
    type Address: Async;
}
