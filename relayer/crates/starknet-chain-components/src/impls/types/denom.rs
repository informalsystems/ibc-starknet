use cgp::prelude::*;
use hermes_chain_type_components::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::denom::ProvideDenomType;

pub struct ProvideTokenAddressDenom;

#[cgp_provider(DenomTypeComponent)]
impl<Chain> ProvideDenomType<Chain> for ProvideTokenAddressDenom
where
    Chain: HasAddressType,
    Chain::Address: Clone,
{
    type Denom = Chain::Address;
}
