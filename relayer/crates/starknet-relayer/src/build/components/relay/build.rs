use core::marker::PhantomData;

use hermes_error::HermesError;
use hermes_relayer_components::build::traits::builders::relay_builder::RelayBuilder;
use cgp::core::field::Index;
use hermes_starknet_chain_components::types::client_id::ClientId as StarknetClientId;
use ibc::core::host::types::identifiers::{ChainId, ClientId as CosmosClientId};

use crate::contexts::builder::{StarknetBuildComponents, StarknetBuilder};
use crate::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;

impl RelayBuilder<StarknetBuilder, Index<0>, Index<1>> for StarknetBuildComponents {
    async fn build_relay(
        build: &StarknetBuilder,
        _index: PhantomData<(Index<0>, Index<1>)>,
        _src_chain_id: &ChainId,
        dst_chain_id: &ChainId,
        src_client_id: &StarknetClientId,
        dst_client_id: &CosmosClientId,
    ) -> Result<StarknetToCosmosRelay, HermesError> {
        let src_chain = build.build_chain().await?;

        let dst_chain = build.cosmos_builder.build_chain(dst_chain_id).await?;

        Ok(
            build.build_starknet_to_cosmos_relay(
                src_chain,
                dst_chain,
                src_client_id,
                dst_client_id,
            ),
        )
    }
}
