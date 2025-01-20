use openzeppelin_testing::declare_class;
use snforge_std::{
    start_cheat_caller_address, start_cheat_block_timestamp_global, start_cheat_block_number_global,
    ContractClass
};
use snforge_std::{spy_events, EventSpy};
use starknet::ContractAddress;
use starknet_ibc_apps::transfer::{ERC20Contract, TRANSFER_PORT_ID};
use starknet_ibc_core::client::ClientContract;
use starknet_ibc_core::router::AppContract;
use starknet_ibc_testkit::configs::{
    TransferAppConfig, TransferAppConfigTrait, CoreConfig, CoreConfigTrait, CometClientConfig,
    CometClientConfigTrait
};
use starknet_ibc_testkit::dummies::{OWNER, CLIENT_TYPE, RELAYER};
use starknet_ibc_testkit::handles::{CoreContract, CoreHandle, AppHandle, ERC20Handle, ClientHandle};

#[derive(Drop, Serde)]
pub struct Setup {
    pub owner: ContractAddress,
    pub erc20_contract_class: ContractClass,
}

#[generate_trait]
pub impl SetupImpl of SetupTrait {
    /// Initializes the test setup with default values.
    fn default() -> Setup {
        Setup { owner: OWNER(), erc20_contract_class: declare_class("ERC20Mintable") }
    }

    /// Deploys an instance of IBC core contract, and sets the owner to the core
    /// address.
    fn deploy_core(ref self: Setup, contract_name: ByteArray) -> CoreContract {
        let core = CoreHandle::deploy(contract_name);
        self.owner = core.address;
        core
    }

    /// Deploys an instance of IBC light client contract.
    fn deploy_client(self: @Setup, contract_name: ByteArray) -> ClientContract {
        ClientHandle::deploy_client(contract_name, *self.owner)
    }

    /// Deploys an instance of ERC20 contract.
    fn deploy_erc20(self: @Setup) -> ERC20Contract {
        ERC20Handle::deploy(*self.erc20_contract_class)
    }

    /// Deploys an instance of ICS-20 Token Transfer contract.
    fn deploy_transfer(self: @Setup, contract_name: ByteArray) -> AppContract {
        AppHandle::deploy_transfer(contract_name, self.owner.clone(), *self.erc20_contract_class)
    }

    /// Sets up testing environment by deploying an instance of IBC core
    /// contract and a light client contract.
    fn setup_core_with_client(
        core_contract_name: ByteArray, client_contract_name: ByteArray
    ) -> (CoreContract, ClientContract) {
        let mut setup = Self::default();

        // Set the block number and timestamp higher than the counterparty's block
        // height and timestamp (typically set to 10 in tests) to avoid timeouts in
        // happy path scenarios.
        start_cheat_block_timestamp_global(20);
        start_cheat_block_number_global(20);

        let mut core = setup.deploy_core(core_contract_name);

        let mut comet = setup.deploy_client(client_contract_name);

        core.register_client(CLIENT_TYPE(), comet.address);

        start_cheat_caller_address(core.address, RELAYER());

        (core, comet)
    }

    /// Sets up testing environment by deploying an instance of ICS-20 Token
    /// Transfer contract and an ERC20 contract.
    fn setup_transfer(transfer_contract_name: ByteArray) -> (AppContract, ERC20Contract) {
        let mut setup = Self::default();

        let mut erc20 = setup.deploy_erc20();

        let ics20 = setup.deploy_transfer(transfer_contract_name);

        // Set the caller address to `OWNER`, as ICS-20 callbacks are permissioned.
        start_cheat_caller_address(ics20.address, OWNER());

        (ics20, erc20)
    }

    /// Sets up a complete testing environment by deploying a full set of
    /// contracts: IBC core, light client, ICS-20 Token Transfer, and ERC20.
    fn setup_full(
        core_contract_name: ByteArray,
        client_contract_name: ByteArray,
        transfer_contract_name: ByteArray
    ) -> (CoreContract, AppContract, ERC20Contract) {
        let mut setup = Self::default();

        // Set the block number and timestamp higher than the counterparty's block
        // height and timestamp (typically set to 10 in tests) to avoid timeouts in
        // happy path scenarios.
        start_cheat_block_timestamp_global(20);
        start_cheat_block_number_global(20);

        let mut core = setup.deploy_core(core_contract_name);

        let comet = setup.deploy_client(client_contract_name);

        core.register_client(CLIENT_TYPE(), comet.address);

        let mut erc20 = setup.deploy_erc20();

        let mut ics20 = setup.deploy_transfer(transfer_contract_name);

        core.register_app(TRANSFER_PORT_ID(), ics20.address);

        start_cheat_caller_address(core.address, RELAYER());

        (core, ics20, erc20)
    }
}

#[derive(Drop)]
pub enum Mode {
    NoClient,
    WithClient,
    WithConnection,
    WithChannel,
}

pub fn setup(
    mode: Mode
) -> (
    CoreContract,
    AppContract,
    ERC20Contract,
    CoreConfig,
    CometClientConfig,
    TransferAppConfig,
    EventSpy
) {
    let mut core_cfg = CoreConfigTrait::default();

    let comet_cfg = CometClientConfigTrait::default();

    let mut transfer_cfg = TransferAppConfigTrait::default();

    let (core, ics20, erc20) = SetupImpl::setup_full("IBCCore", "CometClient", "TransferApp");

    transfer_cfg.set_native_denom(erc20.address);

    let spy = spy_events();

    match mode {
        Mode::NoClient => {},
        Mode::WithClient => { comet_cfg.create_client(@core); },
        Mode::WithConnection => {
            comet_cfg.create_client(@core);
            core_cfg.create_connection(@core);
        },
        Mode::WithChannel => {
            comet_cfg.create_client(@core);
            core_cfg.create_connection(@core);
            core_cfg.create_channel(@core);
        }
    }

    (core, ics20, erc20, core_cfg, comet_cfg, transfer_cfg, spy)
}
