use hermes_core::chain_components::traits::{BlockTypeComponent, ProvideBlockType};
use hermes_prelude::*;

use crate::types::StarknetChainStatus;

pub struct ProvideStarknetBlockType;

#[cgp_provider(BlockTypeComponent)]
impl<Chain> ProvideBlockType<Chain> for ProvideStarknetBlockType
where
    Chain: Async,
{
    type Block = StarknetChainStatus;
}
