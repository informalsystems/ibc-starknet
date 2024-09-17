use openzeppelin_testing::declare_and_deploy;
use starknet_ibc_core::client::ClientContract;

#[generate_trait]
pub impl ClientHandleImpl of ClientHandle {
    fn setup_cometbft() -> ClientContract {
        let mut call_data = array![];

        let address = declare_and_deploy("CometClient", call_data);

        ClientContract { address }
    }
}
