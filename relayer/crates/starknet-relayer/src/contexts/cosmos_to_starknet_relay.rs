use alloc::collections::BTreeSet;
use alloc::sync::Arc;
use core::ops::Deref;

use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeAtComponent,
};
use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::impls::packet_lock::PacketMutexOf;
use hermes_relayer_components::relay::impls::selector::SelectRelayAToB;
use hermes_relayer_components::relay::traits::auto_relayer::CanAutoRelayTarget;
use hermes_relayer_components::relay::traits::chains::{
    CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds,
};
use hermes_relayer_components::relay::traits::channel::open_ack::CanRelayChannelOpenAck;
use hermes_relayer_components::relay::traits::channel::open_confirm::CanRelayChannelOpenConfirm;
use hermes_relayer_components::relay::traits::channel::open_try::CanRelayChannelOpenTry;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::connection::open_ack::CanRelayConnectionOpenAck;
use hermes_relayer_components::relay::traits::connection::open_confirm::CanRelayConnectionOpenConfirm;
use hermes_relayer_components::relay::traits::connection::open_init::CanInitConnection;
use hermes_relayer_components::relay::traits::connection::open_try::CanRelayConnectionOpenTry;
use hermes_relayer_components::relay::traits::event_relayer::CanRelayEvent;
use hermes_relayer_components::relay::traits::ibc_message_sender::{
    IbcMessageSenderComponent, MainSink,
};
use hermes_relayer_components::relay::traits::packet_lock::HasPacketLock;
use hermes_relayer_components::relay::traits::packet_relayer::CanRelayPacket;
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
            ChainTypeAtComponent<Src>,
            ChainTypeAtComponent<Dst>,
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
    + HasLogger
    + CanRelayEvent<SourceTarget>
    + CanRelayEvent<DestinationTarget>
    + CanAutoRelayTarget<SourceTarget>
    + CanAutoRelayTarget<DestinationTarget>
    + CanUseComponent<IbcMessageSenderComponent<MainSink>, (MainSink, SourceTarget)>
    + CanUseComponent<IbcMessageSenderComponent<MainSink>, (MainSink, DestinationTarget)>
{
}

impl CanUseCosmosToStarknetRelay for CosmosToStarknetRelay {}
