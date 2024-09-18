use starknet::ContractAddress;
use starknet_ibc_core::channel::{
    IAppCallbackDispatcher, IAppCallbackDispatcherTrait, Packet, Acknowledgement
};

#[derive(Clone, Debug, Drop, Serde)]
pub struct ApplicationContract {
    pub address: ContractAddress,
}

impl ContractAddressIntoClientAddr of Into<ContractAddress, ApplicationContract> {
    fn into(self: ContractAddress) -> ApplicationContract {
        ApplicationContract { address: self }
    }
}

impl ApplicationContractIntoFelt252 of Into<ApplicationContract, felt252> {
    fn into(self: ApplicationContract) -> felt252 {
        self.address.into()
    }
}

#[generate_trait]
pub impl ApplicationContractImpl of ApplicationContractTrait {
    fn on_recv_packet(self: @ApplicationContract, packet: Packet) -> Acknowledgement {
        IAppCallbackDispatcher { contract_address: *self.address }.on_recv_packet(packet)
    }
}
