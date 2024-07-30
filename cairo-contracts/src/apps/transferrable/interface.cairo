#[starknet::interface]
pub trait ITransferrable<TContractState> {
    fn can_send(self: @TContractState);
    fn can_receive(self: @TContractState);
}

