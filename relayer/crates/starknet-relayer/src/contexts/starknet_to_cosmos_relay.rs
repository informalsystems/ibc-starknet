use alloc::collections::BTreeSet;
use alloc::sync::Arc;
use core::ops::Deref;

use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::components::default::relay::{
    AutoRelayerComponent, AutoRelayerWithHeightsComponent, EventRelayerComponent, MainSink,
};
use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeAtComponent,
};
use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::impls::packet_lock::PacketMutexOf;
use hermes_relayer_components::relay::impls::selector::SelectRelayBToA;
use hermes_relayer_components::relay::traits::chains::{
    CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds,
};
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::connection::open_ack::CanRelayConnectionOpenAck;
use hermes_relayer_components::relay::traits::connection::open_confirm::CanRelayConnectionOpenConfirm;
use hermes_relayer_components::relay::traits::connection::open_init::CanInitConnection;
use hermes_relayer_components::relay::traits::connection::open_try::CanRelayConnectionOpenTry;
use hermes_relayer_components::relay::traits::ibc_message_sender::{
    CanSendIbcMessages, CanSendSingleIbcMessage,
};
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

use crate::presets::relay::{IsStarknetCommonRelayContextPreset, StarknetCommonRelayContextPreset};

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
    ) -> Self {
        Self {
            fields: Arc::new(StarknetToCosmosRelayFields {
                runtime,
                chain_a: dst_chain,
                chain_b: src_chain,
                client_id_a: dst_client_id,
                client_id_b: src_client_id,
                packet_lock_mutex: Arc::new(Mutex::new(BTreeSet::new())),
            }),
        }
    }
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
    + CanUseComponent<AutoRelayerComponent, SourceTarget>
    + CanUseComponent<AutoRelayerComponent, DestinationTarget>
    + CanUseComponent<AutoRelayerWithHeightsComponent, SourceTarget>
    + CanUseComponent<AutoRelayerWithHeightsComponent, DestinationTarget>
{
}

impl CanUseStarknetToCosmosRelay for StarknetToCosmosRelay {}
