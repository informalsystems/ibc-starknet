use cgp::prelude::*;
use hermes_chain_components::traits::{BlockTypeComponent, ProvideBlockType};

use crate::types::status::StarknetChainStatus;

pub struct ProvideStarknetBlockType;

#[cgp_provider(BlockTypeComponent)]
impl<Chain> ProvideBlockType<Chain> for ProvideStarknetBlockType
where
    Chain: Async,
{
    type Block = StarknetChainStatus;
}
