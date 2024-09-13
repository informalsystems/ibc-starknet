#[starknet::contract]
pub(crate) mod MockChannelHandler {
    use starknet_ibc_core::channel::{ChannelHandlerComponent, ChannelEventEmitterComponent};

    component!(
        path: ChannelEventEmitterComponent,
        storage: channel_emitter,
        event: ChannelEventEmitterEvent
    );
    component!(path: ChannelHandlerComponent, storage: channel_handler, event: ChannelHandlerEvent);

    impl ChannelInitializerImpl = ChannelHandlerComponent::ChannelInitializerImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        channel_emitter: ChannelEventEmitterComponent::Storage,
        #[substorage(v0)]
        channel_handler: ChannelHandlerComponent::Storage
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        ChannelEventEmitterEvent: ChannelEventEmitterComponent::Event,
        #[flat]
        ChannelHandlerEvent: ChannelHandlerComponent::Event
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.channel_handler.initializer();
    }
}
