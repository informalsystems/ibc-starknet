use openzeppelin_testing::declare_and_deploy;
use starknet_ibc_core::client::ClientContract;

#[generate_trait]
pub impl ClientHandleImpl of ClientHandle {
    fn deploy_client(contract_name: ByteArray) -> ClientContract {
        let mut call_data = array![];

        let address = declare_and_deploy(contract_name, call_data);

        ClientContract { address }
    }
}
