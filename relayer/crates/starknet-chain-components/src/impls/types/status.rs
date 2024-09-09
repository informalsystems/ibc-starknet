use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::status::ProvideChainStatusType;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimestampType;

use crate::types::status::StarknetChainStatus;

pub struct ProvideStarknetChainStatusType;

impl<Chain> ProvideChainStatusType<Chain> for ProvideStarknetChainStatusType
where
    Chain: HasHeightType<Height = u64> + HasTimestampType<Timestamp = ()>,
{
    type ChainStatus = StarknetChainStatus;

    fn chain_status_height(status: &StarknetChainStatus) -> &u64 {
        &status.block_number
    }

    fn chain_status_timestamp(_status: &Self::ChainStatus) -> &() {
        &()
    }
}
