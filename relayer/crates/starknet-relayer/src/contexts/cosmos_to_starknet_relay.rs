use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;

#[derive(Clone, HasField)]
pub struct CosmosToStarknetRelay {
    pub src_chain: CosmosChain,
    pub dst_chain: StarknetChain,
}
