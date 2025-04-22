use openzeppelin_testing::{EventSpyExt, EventSpyQueue};
use starknet::ContractAddress;
use starknet_ibc_core::client::ClientEventEmitterComponent::{
    CreateClientEvent, Event, MisbehaviourEvent, UpdateClientEvent,
};
use starknet_ibc_core::client::Height;
use starknet_ibc_core::host::ClientId;

#[generate_trait]
pub impl ClientEventSpyExtImpl of ClientEventSpyExt {
    fn assert_create_client_event(
        ref self: EventSpyQueue,
        contract_address: ContractAddress,
        client_id: ClientId,
        consensus_height: Height,
    ) {
        let expected = Event::CreateClientEvent(CreateClientEvent { client_id, consensus_height });
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_update_client_event(
        ref self: EventSpyQueue,
        contract_address: ContractAddress,
        client_id: ClientId,
        consensus_heights: Array<Height>,
        header: Array<felt252>,
    ) {
        let expected = Event::UpdateClientEvent(
            UpdateClientEvent { client_id, consensus_heights, header },
        );
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_misbehaviour_client_event(
        ref self: EventSpyQueue, contract_address: ContractAddress, client_id: ClientId,
    ) {
        let expected = Event::MisbehaviourEvent(MisbehaviourEvent { client_id });
        self.assert_emitted_single(contract_address, expected);
    }
}
