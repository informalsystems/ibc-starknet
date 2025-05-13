use hermes_core::chain_type_components::traits::{
    DenomTypeComponent, HasAddressType, ProvideDenomType,
};
use hermes_prelude::*;

pub struct ProvideTokenAddressDenom;

#[cgp_provider(DenomTypeComponent)]
impl<Chain> ProvideDenomType<Chain> for ProvideTokenAddressDenom
where
    Chain: HasAddressType,
    Chain::Address: Clone,
{
    type Denom = Chain::Address;
}
