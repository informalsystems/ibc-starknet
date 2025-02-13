#[starknet::contract]
pub mod MockConnectionHandler {
    use starknet_ibc_core::connection::{
        ConnectionEventEmitterComponent, ConnectionHandlerComponent,
    };

    component!(
        path: ConnectionEventEmitterComponent,
        storage: channel_emitter,
        event: ConnectionEventEmitterEvent,
    );
    component!(
        path: ConnectionHandlerComponent, storage: channel_handler, event: ConnectionHandlerEvent,
    );

    #[storage]
    struct Storage {
        #[substorage(v0)]
        channel_emitter: ConnectionEventEmitterComponent::Storage,
        #[substorage(v0)]
        channel_handler: ConnectionHandlerComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        ConnectionEventEmitterEvent: ConnectionEventEmitterComponent::Event,
        #[flat]
        ConnectionHandlerEvent: ConnectionHandlerComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {}
}
