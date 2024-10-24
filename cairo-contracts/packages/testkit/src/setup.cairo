use openzeppelin_testing::declare_class;
use snforge_std::{
    start_cheat_block_timestamp_global, start_cheat_block_number_global, ContractClass
};
use starknet::ContractAddress;
use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_core::client::ClientContract;
use starknet_ibc_testkit::dummies::OWNER;
use starknet_ibc_testkit::handles::{
    AppContract, CoreContract, CoreHandle, AppHandle, ERC20Handle, ClientHandle
};

#[derive(Drop, Serde)]
pub struct Setup {
    pub owner: ContractAddress,
    pub erc20_contract_class: ContractClass,
    pub sn_height: u64,
    pub sn_timestamp: u64,
}

#[generate_trait]
pub impl SetupImpl of SetupTrait {
    /// Initializes the test setup with default values.
    fn default() -> Setup {
        // Set the block number and timestamp higher than the counterparty's block
        // height and timestamp (typically set to 10 in tests) to avoid timeouts in
        // happy path scenarios.
        start_cheat_block_timestamp_global(15);
        start_cheat_block_number_global(15);

        Setup {
            owner: OWNER(),
            erc20_contract_class: declare_class("ERC20Mintable"),
            sn_height: 10,
            sn_timestamp: 10
        }
    }

    /// Deploys an instance of IBC core contract, and sets the owner to the core
    /// address.
    fn deploy_core(ref self: Setup) -> CoreContract {
        let core = CoreHandle::deploy();
        self.owner = core.address;
        core
    }

    /// Deploys an instance of CometBFT client contract.
    fn deploy_cometbft(self: @Setup) -> ClientContract {
        ClientHandle::deploy_cometbft()
    }

    /// Deploys an instance of ERC20 contract.
    fn deploy_erc20(self: @Setup) -> ERC20Contract {
        ERC20Handle::deploy(*self.erc20_contract_class)
    }

    /// Deploys an instance of ICS-20 Token Transfer contract.
    fn deploy_transfer(self: @Setup) -> AppContract {
        AppHandle::deploy_transfer("TransferApp", self.owner.clone(), *self.erc20_contract_class)
    }

    /// Deploys an instance of mock ICS-20 Token Transfer contract.
    fn deploy_mock_transfer(self: @Setup) -> AppContract {
        AppHandle::deploy_transfer(
            "MockTransferApp", self.owner.clone(), *self.erc20_contract_class
        )
    }
}

