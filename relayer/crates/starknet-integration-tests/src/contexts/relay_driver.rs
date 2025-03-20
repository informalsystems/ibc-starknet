use cgp::prelude::*;
use hermes_starknet_relayer::contexts::starknet_cosmos_birelay::StarknetCosmosBiRelay;

#[derive(HasField)]
pub struct StarknetRelayDriver {
    pub birelay: StarknetCosmosBiRelay,
}
