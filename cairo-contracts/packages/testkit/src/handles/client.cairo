use openzeppelin_testing::declare_and_deploy;
use snforge_std::ContractClass;
use starknet::ContractAddress;
use starknet_ibc_core::client::ClientContract;

#[generate_trait]
pub impl ClientHandleImpl of ClientHandle {
    fn deploy_client(
        contract_name: ByteArray,
        owner: ContractAddress,
        comet_lib_class: ContractClass,
        ics23_lib_class: ContractClass,
        protobuf_lib_class: ContractClass,
    ) -> ClientContract {
        let mut call_data = array![];

        (owner, comet_lib_class, ics23_lib_class, protobuf_lib_class).serialize(ref call_data);

        let address = declare_and_deploy(contract_name, call_data);

        ClientContract { address }
    }
}
