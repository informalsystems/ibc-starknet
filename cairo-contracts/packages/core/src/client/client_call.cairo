use starknet::ContractAddress;
use starknet_ibc_core::client::{
    ClientErrors, CreateResponse, Height, HeightPartialOrd, IClientHandlerDispatcher,
    IClientHandlerDispatcherTrait, IClientQueryDispatcher, IClientQueryDispatcherTrait,
    IClientStateValidationDispatcher, IClientStateValidationDispatcherTrait, MsgCreateClient,
    MsgRecoverClient, MsgUpdateClient, MsgUpgradeClient, Status, StatusTrait, Timestamp,
    UpdateResponse,
};
use starknet_ibc_core::commitment::{StateProof, StateRoot, StateValue};

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

    fn consensus_state_root(
        self: @ClientContract, client_sequence: u64, height: Height,
    ) -> StateRoot {
        IClientQueryDispatcher { contract_address: *self.address }
            .consensus_state_root(client_sequence, height)
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
        paths: Array<ByteArray>,
        value: StateValue,
        proof: StateProof,
        root: StateRoot,
    ) {
        IClientStateValidationDispatcher { contract_address: *self.address }
            .verify_membership(client_sequence, paths, value, proof, root)
    }

    fn verify_non_membership(
        self: @ClientContract,
        client_sequence: u64,
        paths: Array<ByteArray>,
        proof: StateProof,
        root: StateRoot,
    ) {
        IClientStateValidationDispatcher { contract_address: *self.address }
            .verify_non_membership(client_sequence, paths, proof, root)
    }
}

#[generate_trait]
pub impl ClientContractHandlerImpl of ClientContractHandlerTrait {
    fn create(ref self: ClientContract, msg: MsgCreateClient) -> CreateResponse {
        IClientHandlerDispatcher { contract_address: self.address }.create_client(msg)
    }

    fn update(ref self: ClientContract, msg: MsgUpdateClient) -> UpdateResponse {
        IClientHandlerDispatcher { contract_address: self.address }.update_client(msg)
    }

    fn recover(ref self: ClientContract, msg: MsgRecoverClient) {
        IClientHandlerDispatcher { contract_address: self.address }.recover_client(msg)
    }

    fn upgrade(ref self: ClientContract, msg: MsgUpgradeClient) {
        IClientHandlerDispatcher { contract_address: self.address }.upgrade_client(msg)
    }
}
