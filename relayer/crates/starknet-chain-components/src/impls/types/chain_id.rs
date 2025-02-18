use cgp::prelude::*;
use hermes_cosmos_chain_components::components::client::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ProvideChainIdType;
use ibc::core::host::types::identifiers::ChainId;

pub struct ProvideFeltChainId;

#[cgp_provider(ChainIdTypeComponent)]
impl<Chain: Async> ProvideChainIdType<Chain> for ProvideFeltChainId {
    type ChainId = ChainId;
}
