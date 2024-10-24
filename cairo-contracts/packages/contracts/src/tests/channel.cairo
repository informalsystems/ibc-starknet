use starknet_ibc_apps::transfer::ERC20Contract;
use starknet_ibc_contracts::tests::setup_full;
use starknet_ibc_core::channel::ChannelEndTrait;
use starknet_ibc_testkit::configs::{TransferAppConfigTrait, CometClientConfigTrait};
use starknet_ibc_testkit::dummies::{COSMOS, STARKNET, OWNER};
use starknet_ibc_testkit::event_spy::TransferEventSpyExt;
use starknet_ibc_testkit::handles::{CoreHandle, AppHandle, ERC20Handle};
use starknet_ibc_testkit::setup::SetupImpl;
use starknet_ibc_utils::ComputeKey;

#[test]
fn test_recv_packet_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, ics20, _, mut comet_cfg, mut transfer_cfg, mut spy) = setup_full();

    // -----------------------------------------------------------
    // Create Client
    // -----------------------------------------------------------

    // Create a `MsgCreateClient` message.
    let msg_create_client = comet_cfg.dummy_msg_create_client();

    // Submit the message and create a client.
    core.create_client(msg_create_client);

    // -----------------------------------------------------------
    // Receive Packet (from Cosmos to Starknet)
    // -----------------------------------------------------------

    let msg = transfer_cfg
        .dummy_msg_recv_packet(transfer_cfg.hosted_denom.clone(), COSMOS(), STARKNET());

    core.recv_packet(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    let prefixed_denom = transfer_cfg.prefix_hosted_denom();

    // Assert the `RecvEvent` emitted.
    spy
        .assert_recv_event(
            ics20.address, COSMOS(), STARKNET(), prefixed_denom.clone(), transfer_cfg.amount, true
        );

    // Fetch the token address.
    let token_address = ics20.ibc_token_address(prefixed_denom.key()).unwrap();

    let erc20: ERC20Contract = token_address.into();

    // Check the balance of the receiver.
    erc20.assert_balance(OWNER(), transfer_cfg.amount);

    // Check the total supply of the ERC20 contract.
    erc20.assert_total_supply(transfer_cfg.amount);

    // Retrieve the channel end on Starknet.
    let chan_end_on_b = core
        .channel_end(msg.packet.port_id_on_b.clone(), msg.packet.chan_id_on_b.clone());

    // Assert the channel end is open.
    assert!(chan_end_on_b.is_open());

    // Retrieve the packet receipt.
    let receipt = core
        .packet_receipt(msg.packet.port_id_on_b, msg.packet.chan_id_on_b, msg.packet.seq_on_a,);

    // Assert the packet receipt is true.
    assert!(receipt);
}

