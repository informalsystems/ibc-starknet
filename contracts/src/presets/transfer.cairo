#[starknet::contract]
pub(crate) mod Transfer {
    use starknet::{ContractAddress, ClassHash};
    use starknet_ibc::apps::transfer::component::ICS20TransferComponent;

    component!(path: ICS20TransferComponent, storage: transfer, event: ICS20TransferEvent);

    #[abi(embed_v0)]
    impl ICS20SendTransferImpl =
        ICS20TransferComponent::SendTransfer<ContractState>;
    #[abi(embed_v0)]
    impl ICS20RecvPacketImpl = ICS20TransferComponent::RecvPacket<ContractState>;
    impl ICS20TransferreableImpl = ICS20TransferComponent::Transferrable<ContractState>;
    impl TransferValidationImpl = ICS20TransferComponent::TransferValidationImpl<ContractState>;
    impl TransferExecutionImpl = ICS20TransferComponent::TransferExecutionImpl<ContractState>;
    impl TransferInternalImpl = ICS20TransferComponent::TransferInternalImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        transfer: ICS20TransferComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        ICS20TransferEvent: ICS20TransferComponent::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState, erc20_class_hash: ClassHash) {
        self.transfer.initializer(erc20_class_hash);
    }
}
