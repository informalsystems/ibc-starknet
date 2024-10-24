use snforge_std::{spy_events, EventSpy};
use starknet_ibc_apps::transfer::{ERC20Contract, TRANSFER_PORT_ID};
use starknet_ibc_core::client::ClientContract;
use starknet_ibc_testkit::configs::{
    TransferAppConfigTrait, TransferAppConfig, CometClientConfigTrait, CometClientConfig
};
use starknet_ibc_testkit::dummies::CLIENT_TYPE;
use starknet_ibc_testkit::handles::{CoreContract, AppContract, CoreHandle};
use starknet_ibc_testkit::setup::SetupImpl;

pub fn setup_full() -> (
    CoreContract, AppContract, ERC20Contract, CometClientConfig, TransferAppConfig, EventSpy
) {
    let mut comet_cfg = CometClientConfigTrait::default();

    let mut transfer_cfg = TransferAppConfigTrait::default();

    let mut setup = SetupImpl::default();

    let mut core = setup.deploy_core();

    let comet = setup.deploy_cometbft();

    core.register_client(CLIENT_TYPE(), comet.address);

    let mut erc20 = setup.deploy_erc20();

    let mut ics20 = setup.deploy_transfer();

    core.register_app(TRANSFER_PORT_ID(), ics20.address);

    transfer_cfg.set_native_denom(erc20.address);

    let mut spy = spy_events();

    (core, ics20, erc20, comet_cfg, transfer_cfg, spy)
}

pub fn setup_client() -> (CoreContract, ClientContract, CometClientConfig, EventSpy) {
    let mut cfg = CometClientConfigTrait::default();

    let mut setup = SetupImpl::default();

    let mut core = setup.deploy_core();

    let mut comet = setup.deploy_cometbft();

    core.register_client(CLIENT_TYPE(), comet.address);

    let mut spy = spy_events();

    (core, comet, cfg, spy)
}
