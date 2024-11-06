use starknet::ContractAddress;
use starknet_ibc_core::client::{
    IClientHandlerDispatcher, IClientQueryDispatcher, IClientQueryDispatcherTrait,
    IClientHandlerDispatcherTrait, IClientStateValidationDispatcher,
    IClientStateValidationDispatcherTrait, MsgCreateClient, MsgUpdateClient, MsgRecoverClient,
    MsgUpgradeClient, CreateResponse, UpdateResponse, Height, HeightPartialOrd, Status, StatusTrait,
    ClientErrors, Timestamp,
};
use starknet_ibc_core::commitment::{StateValue, StateProof};

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
        IClientQueryDispatcher { contract_address: *self.address }.client_type()
    }

    fn latest_height(self: @ClientContract, client_sequence: u64) -> Height {
        IClientQueryDispatcher { contract_address: *self.address }.latest_height(client_sequence)
    }

    fn latest_timestamp(self: @ClientContract, client_sequence: u64) -> Timestamp {
        IClientQueryDispatcher { contract_address: *self.address }.latest_timestamp(client_sequence)
    }

    fn status(self: @ClientContract, client_sequence: u64) -> Status {
        IClientQueryDispatcher { contract_address: *self.address }.status(client_sequence)
    }

    fn verify_is_active(self: @ClientContract, client_sequence: u64) {
        let client_status = self.status(client_sequence);
        assert(client_status.is_active(), ClientErrors::INACTIVE_CLIENT);
    }

    fn verify_proof_height(self: @ClientContract, proof_height: @Height, client_sequence: u64) {
        let client_latest_height = self.latest_height(client_sequence);
        assert(proof_height >= @client_latest_height, ClientErrors::INVALID_PROOF_HEIGHT);
    }

    fn verify_membership(
        self: @ClientContract,
        client_sequence: u64,
        path: ByteArray,
        value: StateValue,
        proof: StateProof,
    ) {
        IClientStateValidationDispatcher { contract_address: *self.address }
            .verify_membership(client_sequence, path, value, proof)
    }

    fn verify_non_membership(
        self: @ClientContract, client_sequence: u64, path: ByteArray, proof: StateProof
    ) {
        IClientStateValidationDispatcher { contract_address: *self.address }
            .verify_non_membership(client_sequence, path, proof)
    }
}

#[generate_trait]
pub impl ClientContractHandlerImpl of ClientContractHandlerTrait {
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
