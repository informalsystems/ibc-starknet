use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::chain_id::ProvideChainIdType;
use starknet::core::types::Felt;

pub struct ProvideFeltChainId;

impl<Chain: Async> ProvideChainIdType<Chain> for ProvideFeltChainId {
    type ChainId = Felt;
}
