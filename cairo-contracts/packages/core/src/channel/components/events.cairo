#[starknet::component]
pub mod ChannelEventEmitterComponent {
    use starknet::ContractAddress;
    use starknet_ibc_core::client::{Height, Timestamp};
    use starknet_ibc_core::host::{PortId, ChannelId, Sequence};

    #[storage]
    struct Storage {}

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        SendPacketEvent: SendPacketEvent,
        ReceivePacketEvent: ReceivePacketEvent,
        WriteAcknowledgementEvent: WriteAcknowledgementEvent,
        AcknowledgePacketEvent: AcknowledgePacketEvent,
        TimeoutPacketEvent: TimeoutPacketEvent,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct SendPacketEvent {
        #[key]
        pub port_id_on_a: PortId,
        #[key]
        pub channel_id_on_a: ChannelId,
        #[key]
        pub port_id_on_b: PortId,
        #[key]
        pub channel_id_on_b: ChannelId,
        #[key]
        pub sequence_on_a: Sequence,
        pub packet_data: Array<felt252>,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct ReceivePacketEvent {
        #[key]
        pub port_id_on_a: PortId,
        #[key]
        pub channel_id_on_a: ChannelId,
        #[key]
        pub port_id_on_b: PortId,
        #[key]
        pub channel_id_on_b: ChannelId,
        #[key]
        pub sequence_on_a: Sequence,
        pub packet_data: Array<felt252>,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct WriteAcknowledgementEvent {
        #[key]
        pub port_id_on_a: PortId,
        #[key]
        pub channel_id_on_a: ChannelId,
        #[key]
        pub port_id_on_b: PortId,
        #[key]
        pub channel_id_on_b: ChannelId,
        #[key]
        pub sequence_on_a: Sequence,
        pub packet_data: Array<felt252>,
        pub acknowledgement: Array<felt252>,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct AcknowledgePacketEvent {
        #[key]
        pub port_id_on_a: PortId,
        #[key]
        pub channel_id_on_a: ChannelId,
        #[key]
        pub port_id_on_b: PortId,
        #[key]
        pub channel_id_on_b: ChannelId,
        #[key]
        pub sequence_on_a: Sequence,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct TimeoutPacketEvent {
        #[key]
        pub port_id_on_a: PortId,
        #[key]
        pub channel_id_on_a: ChannelId,
        #[key]
        pub port_id_on_b: PortId,
        #[key]
        pub channel_id_on_b: ChannelId,
        #[key]
        pub sequence_on_a: Sequence,
        #[key]
        pub timeout_height: Height,
        #[key]
        pub timeout_timestamp: Timestamp,
    }

    #[generate_trait]
    pub impl ChannelEventEmitterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ChannelEventEmitterTrait<TContractState> {}
}

