use cgp::prelude::*;
use hermes_chain_components::traits::types::status::ChainStatusTypeComponent;
use hermes_chain_type_components::traits::types::time::HasTimeType;
use hermes_cosmos_chain_components::types::status::Time;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::status::ProvideChainStatusType;

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
