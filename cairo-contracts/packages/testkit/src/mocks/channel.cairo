#[starknet::contract]
pub mod MockChannelHandler {
    use starknet_ibc_core::channel::{ChannelEventEmitterComponent, ChannelHandlerComponent};

    component!(
        path: ChannelEventEmitterComponent,
        storage: channel_emitter,
        event: ChannelEventEmitterEvent,
    );
    component!(path: ChannelHandlerComponent, storage: channel_handler, event: ChannelHandlerEvent);

    #[storage]
    struct Storage {
        #[substorage(v0)]
        channel_emitter: ChannelEventEmitterComponent::Storage,
        #[substorage(v0)]
        channel_handler: ChannelHandlerComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        ChannelEventEmitterEvent: ChannelEventEmitterComponent::Event,
        #[flat]
        ChannelHandlerEvent: ChannelHandlerComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {}
}
