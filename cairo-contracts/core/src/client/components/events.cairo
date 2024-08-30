#[starknet::component]
pub mod ClientEventEmitterComponent {
    use starknet::ContractAddress;
    use starknet_ibc_core::client::{
        MsgCreateClient, MsgUpdateClient, MsgRecoverClient, MsgUpgradeClient, Height,
        CreateResponse, UpdateResponse, ClientErrors
    };
    use starknet_ibc_core::client::{ClientContract, ClientContractTrait};
    use starknet_ibc_core::host::{ClientId, ClientIdImpl};

    #[storage]
    struct Storage {}

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        CreateClientEvent: CreateClientEvent,
        UpdateClientEvent: UpdateClientEvent,
        MisbehaviourEvent: MisbehaviourEvent,
        RecoverClientEvent: RecoverClientEvent,
        UpgradeClientEvent: UpgradeClientEvent,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct CreateClientEvent {
        #[key]
        pub client_id: ClientId,
        #[key]
        pub consensus_height: Height,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct UpdateClientEvent {
        #[key]
        pub client_id: ClientId,
        pub consensus_heights: Array<Height>,
        pub header: Array<felt252>,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct MisbehaviourEvent {
        #[key]
        pub client_id: ClientId,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct RecoverClientEvent {}

    #[derive(Debug, Drop, starknet::Event)]
    pub struct UpgradeClientEvent {}

    #[generate_trait]
    pub impl ClientEventImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ClientEventTrait<TContractState> {
        fn emit_create_client_event(
            ref self: ComponentState<TContractState>, create_resp: CreateResponse
        ) {
            self
                .emit(
                    CreateClientEvent {
                        client_id: create_resp.client_id, consensus_height: create_resp.height,
                    }
                );
        }

        fn emit_update_client_event(
            ref self: ComponentState<TContractState>,
            client_id: ClientId,
            update_heights: Array<Height>,
            client_message: Array<felt252>
        ) {
            self
                .emit(
                    UpdateClientEvent {
                        client_id, consensus_heights: update_heights, header: client_message,
                    }
                );
        }

        fn emit_misbehaviour_event(ref self: ComponentState<TContractState>, client_id: ClientId) {
            self.emit(MisbehaviourEvent { client_id });
        }

        fn emit_recover_client_event(ref self: ComponentState<TContractState>) {
            self.emit(RecoverClientEvent {});
        }

        fn emit_upgrade_client_event(ref self: ComponentState<TContractState>) {
            self.emit(UpgradeClientEvent {});
        }
    }
}

