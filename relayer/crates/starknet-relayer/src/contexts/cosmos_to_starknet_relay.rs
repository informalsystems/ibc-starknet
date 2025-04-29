use alloc::collections::BTreeSet;
use alloc::sync::Arc;
use core::ops::Deref;

use futures::lock::Mutex;
use hermes_core::relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeProviderAtComponent,
};
use hermes_core::relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_core::relayer_components::multi::types::tags::{Dst, Src};
use hermes_core::relayer_components::relay::impls::{
    CanBootstrapChannel, CanBootstrapConnection, PacketMutexOf, SelectRelayAToB,
};
use hermes_core::relayer_components::relay::traits::{
    CanAutoRelayTarget, CanBuildTargetUpdateClientMessage, CanCreateClient, CanInitConnection,
    CanRaiseRelayChainErrors, CanRelayChannelOpenAck, CanRelayChannelOpenConfirm,
    CanRelayChannelOpenTry, CanRelayConnectionOpenAck, CanRelayConnectionOpenConfirm,
    CanRelayConnectionOpenTry, CanRelayEvent, CanRelayPacket, CanSendTargetUpdateClientMessage,
    DestinationTarget, HasDestinationTargetChainTypes, HasPacketLock, HasRelayChains,
    HasRelayClientIds, HasSourceTargetChainTypes, HasTargetClientIds, IbcMessageSenderComponent,
    MainSink, SourceTarget,
};
use hermes_cosmos::relayer::contexts::CosmosChain;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_prelude::*;
use hermes_starknet_chain_components::types::ClientId as StarknetClientId;
use hermes_starknet_chain_context::contexts::StarknetChain;
use ibc::core::host::types::identifiers::ClientId as CosmosClientId;

use crate::presets::relay::StarknetCommonRelayContextPreset;

#[cgp_context(CosmosToStarknetRelayComponents: StarknetCommonRelayContextPreset)]
#[derive(Clone)]
pub struct CosmosToStarknetRelay {
    pub fields: Arc<dyn HasCosmosToStarknetRelayFields>,
}

#[derive(HasField)]
pub struct CosmosToStarknetRelayFields {
    pub runtime: HermesRuntime,
    pub chain_a: CosmosChain,
    pub chain_b: StarknetChain,
    pub client_id_a: CosmosClientId,
    pub client_id_b: StarknetClientId,
    pub packet_lock_mutex: PacketMutexOf<CosmosToStarknetRelay>,
}

pub trait HasCosmosToStarknetRelayFields: Send + Sync + 'static {
    fn fields(&self) -> &CosmosToStarknetRelayFields;
}

impl HasCosmosToStarknetRelayFields for CosmosToStarknetRelayFields {
    fn fields(&self) -> &CosmosToStarknetRelayFields {
        self
    }
}

impl Deref for CosmosToStarknetRelay {
    type Target = CosmosToStarknetRelayFields;

    fn deref(&self) -> &Self::Target {
        self.fields.fields()
    }
}

impl CosmosToStarknetRelay {
    pub fn new(
        runtime: HermesRuntime,
        src_chain: CosmosChain,
        dst_chain: StarknetChain,
        src_client_id: CosmosClientId,
        dst_client_id: StarknetClientId,
    ) -> Self {
        Self {
            fields: Arc::new(CosmosToStarknetRelayFields {
                runtime,
                chain_a: src_chain,
                chain_b: dst_chain,
                client_id_a: src_client_id,
                client_id_b: dst_client_id,
                packet_lock_mutex: Arc::new(Mutex::new(BTreeSet::new())),
            }),
        }
    }
}

delegate_components! {
    CosmosToStarknetRelayComponents {
        [
            ChainTypeProviderAtComponent<Src>,
            ChainTypeProviderAtComponent<Dst>,
            ChainGetterAtComponent<Src>,
            ChainGetterAtComponent<Dst>,
            ClientIdAtGetterComponent<Src, Dst>,
            ClientIdAtGetterComponent<Dst, Src>,
        ]:
            SelectRelayAToB,
    }
}

pub trait CanUseCosmosToStarknetRelay:
    Async
    + HasRelayChains<SrcChain = CosmosChain, DstChain = StarknetChain>
    + HasRelayClientIds
    + CanRaiseRelayChainErrors
    + HasSourceTargetChainTypes
    + HasDestinationTargetChainTypes
    + HasTargetClientIds<SourceTarget>
    + HasTargetClientIds<DestinationTarget>
    + CanCreateClient<DestinationTarget>
    + CanCreateClient<SourceTarget>
    + CanBuildTargetUpdateClientMessage<SourceTarget>
    + CanBuildTargetUpdateClientMessage<DestinationTarget>
    + CanSendTargetUpdateClientMessage<SourceTarget>
    + CanSendTargetUpdateClientMessage<DestinationTarget>
    + CanInitConnection
    + CanRelayConnectionOpenTry
    + CanRelayConnectionOpenAck
    + CanRelayConnectionOpenConfirm
    + CanBootstrapConnection
    + CanRelayChannelOpenTry
    + CanRelayChannelOpenAck
    + CanRelayChannelOpenConfirm
    + CanBootstrapChannel
    + CanRelayPacket
    + HasPacketLock
    + CanRelayEvent<SourceTarget>
    + CanRelayEvent<DestinationTarget>
    + CanAutoRelayTarget<SourceTarget>
    + CanAutoRelayTarget<DestinationTarget>
    + CanUseComponent<IbcMessageSenderComponent<MainSink>, (MainSink, SourceTarget)>
    + CanUseComponent<IbcMessageSenderComponent<MainSink>, (MainSink, DestinationTarget)>
{
}

impl CanUseCosmosToStarknetRelay for CosmosToStarknetRelay {}
