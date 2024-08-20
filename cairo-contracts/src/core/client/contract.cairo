use core::num::traits::Zero;
use starknet::ContractAddress;
use starknet_ibc::core::client::interface::{
    IClientHandler, IClientHandlerDispatcher, IClientStateDispatcher, IClientStateDispatcherTrait,
    IClientHandlerDispatcherTrait, IClientStateValidation, IClientStateValidationDispatcher,
    IClientStateValidationDispatcherTrait,
};
use starknet_ibc::core::client::msgs::{
    MsgCreateClient, MsgUpdateClient, MsgRecoverClient, MsgUpgradeClient
};
use starknet_ibc::core::client::{UpdateResult, Height};

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct ClientContract {
    pub address: ContractAddress,
}

impl ContractAddressIntoClientAddr of Into<ContractAddress, ClientContract> {
    fn into(self: ContractAddress) -> ClientContract {
        ClientContract { address: self }
    }
}

impl ClientContractIntoFelt252 of Into<ClientContract, felt252> {
    fn into(self: ClientContract) -> felt252 {
        self.address.into()
    }
}

pub trait ClientContractTrait {
    fn is_non_zero(self: @ClientContract) -> bool;

    fn client_type(self: @ClientContract) -> felt252;

    fn latest_height(self: @ClientContract, client_sequence: u64) -> Height;

    fn create(ref self: ClientContract, msg: MsgCreateClient, client_sequence: u64);

    fn update(ref self: ClientContract, msg: MsgUpdateClient,) -> UpdateResult;

    fn recover(ref self: ClientContract, msg: MsgRecoverClient,);

    fn upgrade(ref self: ClientContract, msg: MsgUpgradeClient,);
}


impl ClientContractImpl of ClientContractTrait {
    fn is_non_zero(self: @ClientContract) -> bool {
        self.address.is_non_zero()
    }

    fn client_type(self: @ClientContract) -> felt252 {
        IClientStateDispatcher { contract_address: *self.address }.client_type()
    }

    fn latest_height(self: @ClientContract, client_sequence: u64) -> Height {
        IClientStateDispatcher { contract_address: *self.address }.latest_height(client_sequence)
    }

    fn create(ref self: ClientContract, msg: MsgCreateClient, client_sequence: u64) {
        IClientHandlerDispatcher { contract_address: self.address }.create(msg, client_sequence)
    }

    fn update(ref self: ClientContract, msg: MsgUpdateClient,) -> UpdateResult {
        IClientHandlerDispatcher { contract_address: self.address }.update(msg)
    }

    fn recover(ref self: ClientContract, msg: MsgRecoverClient,) {
        IClientHandlerDispatcher { contract_address: self.address }.recover(msg)
    }

    fn upgrade(ref self: ClientContract, msg: MsgUpgradeClient,) {
        IClientHandlerDispatcher { contract_address: self.address }.upgrade(msg)
    }
}
