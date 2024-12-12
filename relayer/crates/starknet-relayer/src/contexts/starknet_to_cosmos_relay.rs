use core::ops::Deref;
use std::sync::Arc;

use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeAtComponent,
};
use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::impls::selector::SelectRelayBToA;
use hermes_relayer_components::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{
    DestinationTarget, HasDestinationTargetChainTypes, HasSourceTargetChainTypes,
    HasTargetClientIds, SourceTarget,
};
use hermes_relayer_components::relay::traits::update_client_message_builder::{
    CanBuildTargetUpdateClientMessage, CanSendTargetUpdateClientMessage,
};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_starknet_chain_components::types::client_id::ClientId as StarknetClientId;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use ibc::core::host::types::identifiers::ClientId as CosmosClientId;

use crate::contexts::cosmos_to_starknet_relay::{
    CosmosToStarknetRelayFields, HasCosmosToStarknetRelayFields,
};
use crate::presets::relay::{IsStarknetCommonRelayContextPreset, StarknetCommonRelayContextPreset};

#[derive(Clone)]
pub struct StarknetToCosmosRelay {
    pub fields: Arc<dyn HasCosmosToStarknetRelayFields>,
}

impl Deref for StarknetToCosmosRelay {
    type Target = CosmosToStarknetRelayFields;

    fn deref(&self) -> &Self::Target {
        self.fields.fields()
    }
}

impl StarknetToCosmosRelay {
    pub fn new(
        runtime: HermesRuntime,
        src_chain: StarknetChain,
        dst_chain: CosmosChain,
        src_client_id: StarknetClientId,
        dst_client_id: CosmosClientId,
    ) -> Self {
        Self {
            fields: Arc::new(CosmosToStarknetRelayFields {
                runtime,
                chain_a: dst_chain,
                chain_b: src_chain,
                client_id_a: dst_client_id,
                client_id_b: src_client_id,
            }),
        }
    }
}

pub struct StarknetToCosmosRelayComponents;

impl HasComponents for StarknetToCosmosRelay {
    type Components = StarknetToCosmosRelayComponents;
}

impl<Name> DelegateComponent<Name> for StarknetToCosmosRelayComponents
where
    Self: IsStarknetCommonRelayContextPreset<Name>,
{
    type Delegate = StarknetCommonRelayContextPreset;
}

delegate_components! {
    StarknetToCosmosRelayComponents {
        [
            ChainTypeAtComponent<Src>,
            ChainTypeAtComponent<Dst>,
            ChainGetterAtComponent<Src>,
            ChainGetterAtComponent<Dst>,
            ClientIdAtGetterComponent<Src, Dst>,
            ClientIdAtGetterComponent<Dst, Src>,
        ]:
            SelectRelayBToA,
    }
}

pub trait CanUseStarknetToCosmosRelay:
    Async
    + HasRelayChains<SrcChain = StarknetChain, DstChain = CosmosChain>
    + HasSourceTargetChainTypes
    + HasDestinationTargetChainTypes
    + HasTargetClientIds<SourceTarget>
    + HasTargetClientIds<DestinationTarget>
    + CanCreateClient<DestinationTarget>
    + CanCreateClient<SourceTarget>
    + CanRaiseRelayChainErrors
    + CanBuildTargetUpdateClientMessage<SourceTarget>
    + CanBuildTargetUpdateClientMessage<DestinationTarget>
    + CanSendTargetUpdateClientMessage<SourceTarget>
    + CanSendTargetUpdateClientMessage<DestinationTarget>
{
}

impl CanUseStarknetToCosmosRelay for StarknetToCosmosRelay {}
