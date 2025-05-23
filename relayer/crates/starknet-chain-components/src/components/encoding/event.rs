#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use hermes_cairo_encoding_components::strategy::ViaCairo;
    pub use hermes_core::encoding_components::traits::DecoderComponent;
    use hermes_core::encoding_components::traits::{EncodedTypeComponent, ProvideEncodedType};
    use hermes_prelude::*;

    use crate::impls::{DecodeOptionalByClassHash, DecodeOptionalByContractAddress};
    use crate::types::{
        AckIbcTransferEvent, AckStatusIbcTransferEvent, AcknowledgePacketEvent, ApprovalEvent,
        ChanOpenAckEvent, ChanOpenConfirmEvent, ChanOpenInitEvent, ChanOpenTryEvent,
        ChannelHandshakeEvents, ConnOpenAckEvent, ConnOpenConfirmEvent, ConnOpenInitEvent,
        ConnOpenTryEvent, ConnectionHandshakeEvents, CreateIbcTokenEvent,
        DecodeChannelHandshakeEvents, DecodeConnectionHandshakeEvents, DecodeErc20Events,
        DecodeIbcTransferEvents, DecodePacketRelayEvents, Erc20Event, IbcTransferEvent,
        PacketRelayEvents, ReceiveIbcTransferEvent, ReceivePacketEvent, SendIbcTransferEvent,
        SendPacketEvent, StarknetEvent, TimeoutIbcTransferEvent, TimeoutPacketEvent, TransferEvent,
        WriteAcknowledgementEvent,
    };

    cgp_preset! {
        StarknetEventEncodingComponents {
            EncodedTypeComponent: ProvideEncodedStarknetEventType,
            DecoderComponent: UseDelegate<StarknetEventEncoderComponents>,
        }
    }

    pub struct ProvideEncodedStarknetEventType;

    #[cgp_provider(EncodedTypeComponent)]
    impl<Encoding: Async> ProvideEncodedType<Encoding> for ProvideEncodedStarknetEventType {
        type Encoded = StarknetEvent;
    }

    pub struct StarknetEventEncoderComponents;

    delegate_components! {
        StarknetEventEncoderComponents {
            [
                (ViaCairo, Erc20Event),
                (ViaCairo, TransferEvent),
                (ViaCairo, ApprovalEvent),
            ]:
                DecodeErc20Events,
            [
                (ViaCairo, IbcTransferEvent),
                (ViaCairo, SendIbcTransferEvent),
                (ViaCairo, ReceiveIbcTransferEvent),
                (ViaCairo, AckIbcTransferEvent),
                (ViaCairo, AckStatusIbcTransferEvent),
                (ViaCairo, TimeoutIbcTransferEvent),
                (ViaCairo, CreateIbcTokenEvent),
            ]:
                DecodeIbcTransferEvents,
            [
                (ViaCairo, ConnectionHandshakeEvents),
                (ViaCairo, ConnOpenInitEvent),
                (ViaCairo, ConnOpenTryEvent),
                (ViaCairo, ConnOpenAckEvent),
                (ViaCairo, ConnOpenConfirmEvent),
            ]:
                DecodeConnectionHandshakeEvents,
            [
                (ViaCairo, ChannelHandshakeEvents),
                (ViaCairo, ChanOpenInitEvent),
                (ViaCairo, ChanOpenTryEvent),
                (ViaCairo, ChanOpenAckEvent),
                (ViaCairo, ChanOpenConfirmEvent),
            ]:
                DecodeChannelHandshakeEvents,
            [
                (ViaCairo, PacketRelayEvents),
                (ViaCairo, SendPacketEvent),
                (ViaCairo, ReceivePacketEvent),
                (ViaCairo, WriteAcknowledgementEvent),
                (ViaCairo, AcknowledgePacketEvent),
                (ViaCairo, TimeoutPacketEvent),
            ]:
                DecodePacketRelayEvents,
            (ViaCairo, Option<Erc20Event>):
                DecodeOptionalByClassHash<symbol!("erc20_hashes")>,
            (ViaCairo, Option<IbcTransferEvent>):
                DecodeOptionalByClassHash<symbol!("ics20_hashes")>,
            [
                (ViaCairo, Option<ConnectionHandshakeEvents>),
                (ViaCairo, Option<ChannelHandshakeEvents>),
                (ViaCairo, Option<PacketRelayEvents>),
            ]:
                DecodeOptionalByContractAddress<symbol!("ibc_core_contract_addresses")>,
        }
    }
}
