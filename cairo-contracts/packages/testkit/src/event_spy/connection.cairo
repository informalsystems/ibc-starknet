use openzeppelin_testing::events::{EventSpyExt, EventSpyExtImpl};
use snforge_std::EventSpy;
use starknet::ContractAddress;
use starknet_ibc_core::connection::ConnectionEventEmitterComponent::{Event, ConnOpenInitEvent};
use starknet_ibc_core::host::{ClientId, ConnectionId};

#[generate_trait]
pub impl ConnectionEventSpyExtImpl of ConnectionEventSpyExt {
    fn assert_conn_open_init_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        client_id_on_a: ClientId,
        connection_id_on_a: ConnectionId,
        client_id_on_b: ClientId,
        connection_id_on_b: ConnectionId,
    ) {
        let expected = Event::ConnOpenInitEvent(
            ConnOpenInitEvent {
                client_id_on_a, connection_id_on_a, client_id_on_b, connection_id_on_b,
            }
        );
        self.assert_emitted_single(contract_address, expected);
    }
}
