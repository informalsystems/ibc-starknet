use openzeppelin_testing::declare_class;
use snforge_std::ContractClass;
use starknet::ContractAddress;
use starknet_ibc_apps::tests::OWNER;
use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_contracts::tests::{
    AppContract, CoreContract, CoreHandle, ClientHandle, ERC20Handle, AppHandle
};
use starknet_ibc_core::client::ClientContract;
use starknet_ibc_core::tests::CLIENT_TYPE;

#[derive(Drop, Serde)]
pub struct Setup {
    pub owner: ContractAddress,
    pub erc20_contract_class: ContractClass
}

#[generate_trait]
pub impl SetupImpl of SetupTrait {
    /// Initializes the test setup with default values.
    fn default() -> Setup {
        Setup { owner: OWNER(), erc20_contract_class: declare_class("ERC20Mintable"), }
    }

    /// Deploys an instance of IBC core contract.
    fn deploy_core(self: @Setup) -> CoreContract {
        CoreHandle::deploy()
    }

    /// Deploys an instance of CometBFT client contract.
    fn deploy_cometbft(self: @Setup, ref core: CoreContract) -> ClientContract {
        // Deploy a Comet client contract.
        let comet = ClientHandle::deploy_cometbft();

        // Register the Comet client into the IBC core contract.
        core.register_client(CLIENT_TYPE(), comet.address);

        comet
    }

    /// Deploys an instance of ERC20 contract.
    fn deploy_erc20(self: @Setup) -> ERC20Contract {
        ERC20Handle::deploy(*self.erc20_contract_class)
    }

    /// Deploys an instance of ICS-20 Token Transfer contract.
    fn deploy_trasnfer(self: @Setup) -> AppContract {
        AppHandle::deploy_transfer(self.owner.clone(), *self.erc20_contract_class)
    }
}

