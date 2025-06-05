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
use hermes_starknet_madara_tests::contexts::MadaraChain;
use ibc::core::host::types::identifiers::ClientId as CosmosClientId;

use crate::presets::MadaraCommonRelayContextPreset;

#[cgp_context(CosmosToMadaraRelayComponents: MadaraCommonRelayContextPreset)]
#[derive(Clone)]
pub struct CosmosToMadaraRelay {
    pub fields: Arc<dyn HasCosmosToMadaraRelayFields>,
}

#[derive(HasField)]
pub struct CosmosToMadaraRelayFields {
    pub runtime: HermesRuntime,
    pub chain_a: CosmosChain,
    pub chain_b: MadaraChain,
    pub client_id_a: CosmosClientId,
    pub client_id_b: StarknetClientId,
    pub packet_lock_mutex: PacketMutexOf<CosmosToMadaraRelay>,
}

pub trait HasCosmosToMadaraRelayFields: Send + Sync + 'static {
    fn fields(&self) -> &CosmosToMadaraRelayFields;
}

impl HasCosmosToMadaraRelayFields for CosmosToMadaraRelayFields {
    fn fields(&self) -> &CosmosToMadaraRelayFields {
        self
    }
}

impl Deref for CosmosToMadaraRelay {
    type Target = CosmosToMadaraRelayFields;

    fn deref(&self) -> &Self::Target {
        self.fields.fields()
    }
}

impl CosmosToMadaraRelay {
    pub fn new(
        runtime: HermesRuntime,
        src_chain: CosmosChain,
        dst_chain: MadaraChain,
        src_client_id: CosmosClientId,
        dst_client_id: StarknetClientId,
    ) -> Self {
        Self {
            fields: Arc::new(CosmosToMadaraRelayFields {
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
    CosmosToMadaraRelayComponents {
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

pub trait CanUseCosmosToMadaraRelay:
    Async
    + HasRelayChains<SrcChain = CosmosChain, DstChain = MadaraChain>
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

impl CanUseCosmosToMadaraRelay for CosmosToMadaraRelay {}
