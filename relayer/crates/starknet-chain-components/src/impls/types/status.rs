use cgp::prelude::*;
use hermes_core::chain_components::traits::{
    ChainStatusTypeComponent, HasHeightType, ProvideChainStatusType,
};
use hermes_core::chain_type_components::traits::HasTimeType;
use hermes_cosmos_chain_components::types::Time;

use crate::types::status::StarknetChainStatus;

pub struct ProvideStarknetChainStatusType;

#[cgp_provider(ChainStatusTypeComponent)]
impl<Chain> ProvideChainStatusType<Chain> for ProvideStarknetChainStatusType
where
    Chain: HasHeightType<Height = u64> + HasTimeType<Time = Time>,
{
    type ChainStatus = StarknetChainStatus;

    fn chain_status_height(status: &StarknetChainStatus) -> &u64 {
        &status.height
    }

    fn chain_status_time(status: &Self::ChainStatus) -> &Time {
        &status.time
    }
}
