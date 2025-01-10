use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::chain_id::ProvideChainIdType;
use ibc::core::host::types::identifiers::ChainId;

pub struct ProvideFeltChainId;

impl<Chain: Async> ProvideChainIdType<Chain> for ProvideFeltChainId {
    type ChainId = ChainId;
}
