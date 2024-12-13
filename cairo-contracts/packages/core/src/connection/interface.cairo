use starknet_ibc_core::connection::{
    MsgConnOpenAck, MsgConnOpenConfirm, MsgConnOpenInit, MsgConnOpenTry, ConnectionEnd,
};
use starknet_ibc_core::host::ConnectionId;

#[starknet::interface]
pub trait IConnectionHandler<TContractState> {
    fn conn_open_init(ref self: TContractState, msg: MsgConnOpenInit) -> ConnectionId;

    fn conn_open_try(ref self: TContractState, msg: MsgConnOpenTry) -> ConnectionId;

    fn conn_open_ack(ref self: TContractState, msg: MsgConnOpenAck);

    fn conn_open_confirm(ref self: TContractState, msg: MsgConnOpenConfirm);
}

#[starknet::interface]
pub trait IConnectionQuery<TContractState> {
    fn connection_end(self: @TContractState, connection_id: ConnectionId) -> ConnectionEnd;
}
