use openzeppelin_testing::events::{EventSpyExt, EventSpyExtImpl};
use snforge_std::EventSpy;
use starknet::ContractAddress;
use starknet_ibc_core::connection::ConnectionEventEmitterComponent::{
    ConnOpenAckEvent, ConnOpenConfirmEvent, ConnOpenInitEvent, ConnOpenTryEvent, Event,
};
use starknet_ibc_core::host::{ClientId, ConnectionId};

#[generate_trait]
pub impl ConnectionEventSpyExtImpl of ConnectionEventSpyExt {
    fn assert_conn_open_init_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        client_id_on_a: ClientId,
        connection_id_on_a: ConnectionId,
        client_id_on_b: ClientId,
    ) {
        let expected = Event::ConnOpenInitEvent(
            ConnOpenInitEvent { client_id_on_a, connection_id_on_a, client_id_on_b },
        );
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_conn_open_try_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        client_id_on_b: ClientId,
        connection_id_on_b: ConnectionId,
        client_id_on_a: ClientId,
        connection_id_on_a: ConnectionId,
    ) {
        let expected = Event::ConnOpenTryEvent(
            ConnOpenTryEvent {
                client_id_on_b, connection_id_on_b, client_id_on_a, connection_id_on_a,
            },
        );
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_conn_open_ack_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        client_id_on_a: ClientId,
        connection_id_on_a: ConnectionId,
        client_id_on_b: ClientId,
        connection_id_on_b: ConnectionId,
    ) {
        let expected = Event::ConnOpenAckEvent(
            ConnOpenAckEvent {
                client_id_on_a, connection_id_on_a, client_id_on_b, connection_id_on_b,
            },
        );
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_conn_open_confirm_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        client_id_on_b: ClientId,
        connection_id_on_b: ConnectionId,
        client_id_on_a: ClientId,
        connection_id_on_a: ConnectionId,
    ) {
        let expected = Event::ConnOpenConfirmEvent(
            ConnOpenConfirmEvent {
                client_id_on_b, connection_id_on_b, client_id_on_a, connection_id_on_a,
            },
        );
        self.assert_emitted_single(contract_address, expected);
    }
}
