use core::num::traits::Zero;
use starknet::ContractAddress;
use starknet_ibc_core::client::{
    IClientHandler, IClientHandlerDispatcher, IClientStateDispatcher, IClientStateDispatcherTrait,
    IClientHandlerDispatcherTrait, IClientStateValidation, IClientStateValidationDispatcher,
    IClientStateValidationDispatcherTrait, MsgCreateClient, MsgUpdateClient, MsgRecoverClient,
    MsgUpgradeClient, CreateResponse, UpdateResponse, Height, Status, StatusTrait, ClientErrors
};
use starknet_ibc_core::host::ClientId;

#[derive(Clone, Debug, Drop, Serde)]
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

#[generate_trait]
pub impl ClientContractImpl of ClientContractTrait {
    fn client_type(self: @ClientContract) -> felt252 {
        IClientStateDispatcher { contract_address: *self.address }.client_type()
    }

    fn latest_height(self: @ClientContract, client_sequence: u64) -> Height {
        IClientStateDispatcher { contract_address: *self.address }.latest_height(client_sequence)
    }

    fn status(self: @ClientContract, client_sequence: u64) -> Status {
        IClientStateDispatcher { contract_address: *self.address }.status(client_sequence)
    }

    fn verify_membership(
        self: @ClientContract,
        client_sequence: u64,
        path: ByteArray,
        value: Array<u8>,
        proof: Array<u8>
    ) {
        IClientStateValidationDispatcher { contract_address: *self.address }
            .verify_membership(client_sequence, path, value, proof)
    }

    fn verify_non_membership(
        self: @ClientContract, client_sequence: u64, path: ByteArray, proof: Array<u8>
    ) {
        IClientStateValidationDispatcher { contract_address: *self.address }
            .verify_non_membership(client_sequence, path, proof)
    }
    fn create(ref self: ClientContract, msg: MsgCreateClient) -> CreateResponse {
        IClientHandlerDispatcher { contract_address: self.address }.create_client(msg)
    }

    fn update(ref self: ClientContract, msg: MsgUpdateClient,) -> UpdateResponse {
        IClientHandlerDispatcher { contract_address: self.address }.update_client(msg)
    }

    fn recover(ref self: ClientContract, msg: MsgRecoverClient,) {
        IClientHandlerDispatcher { contract_address: self.address }.recover_client(msg)
    }

    fn upgrade(ref self: ClientContract, msg: MsgUpgradeClient,) {
        IClientHandlerDispatcher { contract_address: self.address }.upgrade_client(msg)
    }
}
