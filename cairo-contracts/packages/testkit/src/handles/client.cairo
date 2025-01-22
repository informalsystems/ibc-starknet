use openzeppelin_testing::declare_and_deploy;
use openzeppelin_utils::serde::SerializedAppend;
use starknet::ContractAddress;
use starknet_ibc_core::client::ClientContract;

#[generate_trait]
pub impl ClientHandleImpl of ClientHandle {
    fn deploy_client(contract_name: ByteArray, owner: ContractAddress) -> ClientContract {
        let mut call_data = array![];
        call_data.append_serde(owner);

        let address = declare_and_deploy(contract_name, call_data);

        ClientContract { address }
    }
}
