use alloc::collections::BTreeSet;
use alloc::sync::Arc;
use core::ops::Deref;
use core::time::Duration;

use futures::lock::Mutex;
use hermes_core::relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeProviderAtComponent,
};
use hermes_core::relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_core::relayer_components::multi::types::tags::{Dst, Src};
use hermes_core::relayer_components::relay::impls::{
    CanBootstrapConnection, PacketMutexOf, SelectRelayBToA,
};
use hermes_core::relayer_components::relay::traits::{
    AutoRelayerWithHeightsComponent, CanBuildTargetUpdateClientMessage, CanCreateClient,
    CanInitConnection, CanRaiseRelayChainErrors, CanRelayConnectionOpenAck,
    CanRelayConnectionOpenConfirm, CanRelayConnectionOpenTry, CanRelayPacket, CanSendIbcMessages,
    CanSendSingleIbcMessage, CanSendTargetUpdateClientMessage, DestinationTarget,
    EventRelayerComponent, HasDestinationTargetChainTypes, HasRelayChains, HasRelayClientIds,
    HasSourceTargetChainTypes, HasTargetClientIds, MainSink, SourceTarget,
    TargetAutoRelayerComponent,
};
use hermes_cosmos::relayer::contexts::CosmosChain;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_prelude::*;
use hermes_starknet_chain_components::types::ClientId as StarknetClientId;
use hermes_starknet_chain_context::contexts::StarknetChain;
use ibc::core::host::types::identifiers::ClientId as CosmosClientId;

use crate::presets::StarknetCommonRelayContextPreset;

#[cgp_context(StarknetToCosmosRelayComponents: StarknetCommonRelayContextPreset)]
#[derive(Clone)]
pub struct StarknetToCosmosRelay {
    pub fields: Arc<dyn HasStarknetToCosmosRelayFields>,
}

#[derive(HasField)]
pub struct StarknetToCosmosRelayFields {
    pub runtime: HermesRuntime,
    pub chain_a: CosmosChain,
    pub chain_b: StarknetChain,
    pub client_id_a: CosmosClientId,
    pub client_id_b: StarknetClientId,
    pub packet_lock_mutex: PacketMutexOf<StarknetToCosmosRelay>,
    pub refresh_rate_a_to_b: Option<Duration>,
    pub refresh_rate_b_to_a: Option<Duration>,
}

pub trait HasStarknetToCosmosRelayFields: Async {
    fn fields(&self) -> &StarknetToCosmosRelayFields;
}

impl HasStarknetToCosmosRelayFields for StarknetToCosmosRelayFields {
    fn fields(&self) -> &StarknetToCosmosRelayFields {
        self
    }
}

impl Deref for StarknetToCosmosRelay {
    type Target = StarknetToCosmosRelayFields;

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
        refresh_rate_a_to_b: Option<Duration>,
        refresh_rate_b_to_a: Option<Duration>,
    ) -> Self {
        Self {
            fields: Arc::new(StarknetToCosmosRelayFields {
                runtime,
                chain_a: dst_chain,
                chain_b: src_chain,
                client_id_a: dst_client_id,
                client_id_b: src_client_id,
                packet_lock_mutex: Arc::new(Mutex::new(BTreeSet::new())),
                refresh_rate_a_to_b,
                refresh_rate_b_to_a,
            }),
        }
    }
}

delegate_components! {
    StarknetToCosmosRelayComponents {
        [
            ChainTypeProviderAtComponent<Src>,
            ChainTypeProviderAtComponent<Dst>,
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
    + HasRelayClientIds
    + CanRaiseRelayChainErrors
    + HasSourceTargetChainTypes
    + HasDestinationTargetChainTypes
    + HasTargetClientIds<SourceTarget>
    + HasTargetClientIds<DestinationTarget>
    + CanCreateClient<DestinationTarget>
    + CanCreateClient<SourceTarget>
    + CanSendSingleIbcMessage<MainSink, SourceTarget>
    + CanSendSingleIbcMessage<MainSink, DestinationTarget>
    + CanBuildTargetUpdateClientMessage<SourceTarget>
    + CanBuildTargetUpdateClientMessage<DestinationTarget>
    + CanSendTargetUpdateClientMessage<SourceTarget>
    + CanSendTargetUpdateClientMessage<DestinationTarget>
    + CanSendIbcMessages<MainSink, SourceTarget>
    + CanSendIbcMessages<MainSink, DestinationTarget>
    + CanInitConnection
    + CanRelayConnectionOpenTry
    + CanRelayConnectionOpenAck
    + CanRelayConnectionOpenConfirm
    + CanBootstrapConnection
    + CanRelayPacket
    + CanUseComponent<EventRelayerComponent, SourceTarget>
    + CanUseComponent<EventRelayerComponent, DestinationTarget>
    + CanUseComponent<TargetAutoRelayerComponent, SourceTarget>
    + CanUseComponent<TargetAutoRelayerComponent, DestinationTarget>
    + CanUseComponent<AutoRelayerWithHeightsComponent, SourceTarget>
    + CanUseComponent<AutoRelayerWithHeightsComponent, DestinationTarget>
{
}

impl CanUseStarknetToCosmosRelay for StarknetToCosmosRelay {}
