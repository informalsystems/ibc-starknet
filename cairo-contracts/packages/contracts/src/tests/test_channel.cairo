use openzeppelin_testing::declare_class;
use starknet_ibc_apps::tests::{TransferAppConfigTrait, OWNER};
use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_contracts::tests::setups::{ClientHandle, ERC20Handle, AppHandle, AppContract};
use starknet_ibc_contracts::tests::setups::{CoreContract, CoreHandle};
use starknet_ibc_core::client::ClientContract;

// Deploys an instance of IBC core, Cometbft ligth client, and Token Transfer
// applicaiton contracts, and registers the client and application into the core
// contract.
fn setup_contracts(
    client_type: felt252
) -> (CoreContract, ClientContract, AppContract, ERC20Contract) {
    // Deploy an IBC core contract.
    let mut ibc = CoreHandle::setup();

    // Deploy a Comet client contract.
    let comet = ClientHandle::setup_cometbft();

    // Register the Comet client into the IBC core contract.
    ibc.register_client(client_type, comet.address);

    // Declare the ERC20 contract class.
    let erc20_contract_class = declare_class("ERC20Mintable");

    // Deploy an ERC20 contract.
    let mut erc20 = ERC20Handle::setup(erc20_contract_class);

    // Deploy an ICS20 Token Transfer contract.
    let mut ics20 = AppHandle::setup_transfer(OWNER(), erc20_contract_class);

    (ibc, comet, ics20, erc20)
}

