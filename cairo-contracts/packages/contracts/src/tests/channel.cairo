use snforge_std::start_cheat_caller_address;
use starknet_ibc_apps::transfer::{ERC20Contract, SUCCESS_ACK, VERSION};
use starknet_ibc_core::channel::{ChannelEndTrait, ChannelOrdering, AckStatus, ChannelState};
use starknet_ibc_core::client::{Height, Timestamp};
use starknet_ibc_core::host::{SequenceImpl, Sequence};
use starknet_ibc_testkit::configs::{
    TransferAppConfigTrait, CoreConfigTrait, CometClientConfigTrait
};
use starknet_ibc_testkit::dummies::{
    HEIGHT, TIMESTAMP, COSMOS, STARKNET, CLIENT_ID, CONNECTION_ID, CHANNEL_ID, PORT_ID, SUPPLY,
    PACKET_COMMITMENT_ON_SN, RELAYER, USER, CS_USER
};
use starknet_ibc_testkit::event_spy::{TransferEventSpyExt, ChannelEventSpyExt};
use starknet_ibc_testkit::handles::{CoreHandle, AppHandle, ERC20Handle};
use starknet_ibc_testkit::setup::{setup, Mode};
use starknet_ibc_utils::ComputeKey;

#[test]
fn test_chan_open_init_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, core_cfg, _, _, mut spy) = setup(Mode::WithConnection);

    // -----------------------------------------------------------
    // Channel Open Init
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_chan_open_init();

    core.chan_open_init(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    spy.assert_chan_open_init_event(core.address, CHANNEL_ID(0), VERSION(), msg.clone());

    let chan_end_on_a = core.channel_end(msg.port_id_on_a, CHANNEL_ID(0));

    assert_eq!(chan_end_on_a.state(), @ChannelState::Init);
}

#[test]
fn test_chan_open_try_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, core_cfg, _, _, mut spy) = setup(Mode::WithConnection);

    // -----------------------------------------------------------
    // Channel Open Init
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_chan_open_try();

    core.chan_open_try(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    spy.assert_chan_open_try_event(core.address, CHANNEL_ID(0), VERSION(), msg.clone());

    let chan_end_on_b = core.channel_end(msg.port_id_on_a, CHANNEL_ID(0));

    assert_eq!(chan_end_on_b.state(), @ChannelState::TryOpen);
}

#[test]
fn test_chan_open_ack_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, core_cfg, _, _, mut spy) = setup(Mode::WithConnection);

    // -----------------------------------------------------------
    // Channel Open Init
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_chan_open_init();

    core.chan_open_init(msg);

    // -----------------------------------------------------------
    // Channel Open Ack
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_chan_open_ack();

    core.chan_open_ack(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    spy.assert_chan_open_ack_event(core.address, PORT_ID(), CONNECTION_ID(0), msg.clone());

    let chan_end_on_a = core.channel_end(msg.port_id_on_a, msg.chan_id_on_a);

    assert_eq!(chan_end_on_a.state(), @ChannelState::Open);
}

#[test]
fn test_chan_open_confirm_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, core_cfg, _, _, mut spy) = setup(Mode::WithConnection);

    // -----------------------------------------------------------
    // Channel Open Try
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_chan_open_try();

    core.chan_open_try(msg.clone());

    // -----------------------------------------------------------
    // Channel Open Confirm
    // -----------------------------------------------------------

    let msg = core_cfg.dummy_msg_chan_open_confirm();

    core.chan_open_confirm(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    spy
        .assert_chan_open_confirm_event(
            core.address, PORT_ID(), CHANNEL_ID(0), CONNECTION_ID(0), msg.clone()
        );

    let chan_end_on_b = core.channel_end(msg.port_id_on_b, msg.chan_id_on_b);

    assert_eq!(chan_end_on_b.state(), @ChannelState::Open);
}

#[test]
fn test_send_packet_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, erc20, _, _, transfer_cfg, mut spy) = setup(Mode::WithChannel);

    // -----------------------------------------------------------
    // Send Packet (from Starknet to Cosmos)
    // -----------------------------------------------------------

    let mut packet = transfer_cfg
        .dummy_packet(transfer_cfg.native_denom.clone(), STARKNET(), COSMOS());

    core.send_packet(packet.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    spy.assert_send_packet_event(core.address, ChannelOrdering::Unordered, packet.clone());

    let chan_end_on_a = core.channel_end(packet.port_id_on_a.clone(), packet.chan_id_on_a.clone());

    assert!(chan_end_on_a.is_open());

    let commitment = core
        .packet_commitment(
            packet.port_id_on_a.clone(), packet.chan_id_on_a.clone(), packet.seq_on_a.clone()
        );

    assert_eq!(commitment, PACKET_COMMITMENT_ON_SN(erc20));

    let next_sequence_send = core.next_sequence_send(packet.port_id_on_a, packet.chan_id_on_a);

    assert_eq!(next_sequence_send, packet.seq_on_a.increment());
}

#[test]
fn test_recv_packet_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, ics20, _, _, _, transfer_cfg, mut spy) = setup(Mode::WithChannel);

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

    // Assert the `RecvEvent` emitted by the ICS20 contract.
    spy
        .assert_recv_event(
            ics20.address, CS_USER(), USER(), prefixed_denom.clone(), transfer_cfg.amount, true
        );

    // Assert the `ReceivePacketEvent` emitted by the core contract.
    spy.assert_recv_packet_event(core.address, ChannelOrdering::Unordered, msg.packet.clone());

    // Fetch the token address.
    let token_address = ics20.ibc_token_address(prefixed_denom.key());

    let erc20: ERC20Contract = token_address.into();

    // Check the balance of the receiver.
    erc20.assert_balance(USER(), transfer_cfg.amount);

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

#[test]
#[should_panic(expected: 'ICS04: missing commitment')]
fn test_successful_ack_packet_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, ics20, mut erc20, _, _, transfer_cfg, mut spy) = setup(Mode::WithChannel);

    // -----------------------------------------------------------
    // Send Packet (from Starknet to Cosmos)
    // -----------------------------------------------------------

    start_cheat_caller_address(ics20.address, USER());

    // User approves the amount of allowance for the `TransferApp` contract.
    erc20.approve(USER(), ics20.address, transfer_cfg.amount);

    let msg_transfer = transfer_cfg
        .dummy_msg_transfer(transfer_cfg.native_denom.clone(), CS_USER());

    // Submit a `MsgTransfer` to the `TransferApp` contract.
    ics20.send_transfer(msg_transfer.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    let next_sequence_send = core
        .next_sequence_send(msg_transfer.port_id_on_a.clone(), msg_transfer.chan_id_on_a.clone());

    let seq_on_a = Sequence { sequence: next_sequence_send.sequence - 1 };

    let commitment = core
        .packet_commitment(
            msg_transfer.port_id_on_a.clone(), msg_transfer.chan_id_on_a.clone(), seq_on_a.clone()
        );

    assert_eq!(commitment, PACKET_COMMITMENT_ON_SN(erc20));

    // Check the balance of the sender.
    erc20.assert_balance(USER(), SUPPLY - transfer_cfg.amount);

    // -----------------------------------------------------------
    // Acknowledge Packet (on Starknet)
    // -----------------------------------------------------------

    start_cheat_caller_address(ics20.address, core.address);

    let msg = transfer_cfg
        .dummy_msg_ack_packet(
            transfer_cfg.native_denom.clone(), STARKNET(), COSMOS(), SUCCESS_ACK()
        );

    core.ack_packet(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    spy
        .assert_ack_event(
            ics20.address,
            USER(),
            CS_USER(),
            transfer_cfg.native_denom.clone(),
            transfer_cfg.amount,
            SUCCESS_ACK()
        );

    spy.assert_ack_status_event(ics20.address, AckStatus::Success(SUCCESS_ACK()));

    spy.assert_ack_packet_event(core.address, ChannelOrdering::Unordered, msg.packet.clone());

    // Check the balance of the sender.
    erc20.assert_balance(USER(), SUPPLY - transfer_cfg.amount);

    core
        .packet_commitment(
            msg_transfer.port_id_on_a.clone(), msg_transfer.chan_id_on_a.clone(), seq_on_a
        );
}

#[test]
#[should_panic(expected: 'ICS04: missing commitment')]
fn test_failure_ack_packet_ok() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, ics20, mut erc20, _, _, transfer_cfg, mut spy) = setup(Mode::WithChannel);

    // -----------------------------------------------------------
    // Send Packet (from Starknet to Cosmos)
    // -----------------------------------------------------------

    start_cheat_caller_address(ics20.address, USER());

    // User approves the amount of allowance for the `TransferApp` contract.
    erc20.approve(USER(), ics20.address, transfer_cfg.amount);

    let msg_transfer = transfer_cfg
        .dummy_msg_transfer(transfer_cfg.native_denom.clone(), CS_USER());

    // Submit a `MsgTransfer` to the `TransferApp` contract.
    ics20.send_transfer(msg_transfer.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    let next_sequence_send = core
        .next_sequence_send(msg_transfer.port_id_on_a.clone(), msg_transfer.chan_id_on_a.clone());

    let seq_on_a = Sequence { sequence: next_sequence_send.sequence - 1 };

    let commitment = core
        .packet_commitment(
            msg_transfer.port_id_on_a.clone(), msg_transfer.chan_id_on_a.clone(), seq_on_a.clone()
        );

    assert_eq!(commitment, PACKET_COMMITMENT_ON_SN(erc20));

    // Check the balance of the sender.
    erc20.assert_balance(USER(), SUPPLY - transfer_cfg.amount);

    // -----------------------------------------------------------
    // Acknowledge Packet (on Starknet)
    // -----------------------------------------------------------

    start_cheat_caller_address(ics20.address, core.address);

    let failure_ack = array![0].into();

    let msg = transfer_cfg
        .dummy_msg_ack_packet(
            transfer_cfg.native_denom.clone(), STARKNET(), COSMOS(), failure_ack.clone()
        );

    core.ack_packet(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    spy
        .assert_ack_event(
            ics20.address,
            USER(),
            CS_USER(),
            transfer_cfg.native_denom.clone(),
            transfer_cfg.amount,
            failure_ack.clone()
        );

    spy.assert_ack_status_event(ics20.address, AckStatus::Error(failure_ack));

    spy.assert_ack_packet_event(core.address, ChannelOrdering::Unordered, msg.packet.clone());

    // Check if the balance of the sender to ensure the refund.
    erc20.assert_balance(USER(), SUPPLY);

    core
        .packet_commitment(
            msg_transfer.port_id_on_a.clone(), msg_transfer.chan_id_on_a.clone(), seq_on_a
        );
}

#[test]
#[should_panic(expected: 'ICS04: missing commitment')]
fn test_ack_packet_for_never_sent_packet() {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, _, _, _, _, transfer_cfg, _) = setup(Mode::WithChannel);

    // -----------------------------------------------------------
    // Acknowledge Packet (on Starknet)
    // -----------------------------------------------------------

    let msg = transfer_cfg
        .dummy_msg_ack_packet(
            transfer_cfg.native_denom.clone(), STARKNET(), COSMOS(), SUCCESS_ACK()
        );

    core.ack_packet(msg);
}

fn try_timeout_packet(timeout_height: Height, timeout_timestamp: Timestamp) {
    // -----------------------------------------------------------
    // Setup Essentials
    // -----------------------------------------------------------

    let (core, ics20, mut erc20, mut core_cfg, comet_cfg, mut transfer_cfg, mut spy) = setup(
        Mode::WithConnection
    );

    let updating_height = HEIGHT(11); // Set to 11 as client is created at height 10.

    let updating_timestamp: u64 = 11; // Set to 11 as client is created at timestamp 10.

    transfer_cfg.set_timeout_height(timeout_height);

    transfer_cfg.set_timeout_timestamp(timeout_timestamp);

    core_cfg.create_channel(@core);

    // -----------------------------------------------------------
    // Send Packet (from Starknet to Cosmos)
    // -----------------------------------------------------------

    start_cheat_caller_address(ics20.address, USER());

    erc20.approve(USER(), ics20.address, transfer_cfg.amount);

    let msg_transfer = transfer_cfg
        .dummy_msg_transfer(transfer_cfg.native_denom.clone(), CS_USER());

    ics20.send_transfer(msg_transfer.clone());

    // -----------------------------------------------------------
    // Update Client
    // -----------------------------------------------------------

    start_cheat_caller_address(ics20.address, core.address);

    core.register_relayer(RELAYER());

    let msg = comet_cfg
        .dummy_msg_update_client(
            CLIENT_ID(), comet_cfg.latest_height, updating_height.clone(), updating_timestamp,
        );

    core.update_client(msg.clone());

    // -----------------------------------------------------------
    // Timeout Packet
    // -----------------------------------------------------------

    let mut msg = transfer_cfg
        .dummy_msg_timeout_packet(
            transfer_cfg.native_denom.clone(), STARKNET(), COSMOS(), updating_height,
        );

    core.timeout_packet(msg.clone());

    // -----------------------------------------------------------
    // Check Results
    // -----------------------------------------------------------

    let packet = msg.packet;

    spy.assert_timeout_packet_event(core.address, ChannelOrdering::Unordered, packet.clone());

    // Check if the balance of the sender to ensure the refund.
    erc20.assert_balance(USER(), SUPPLY);

    core
        .packet_commitment(
            packet.port_id_on_a.clone(), packet.chan_id_on_a.clone(), packet.seq_on_a
        );
}

#[test]
#[should_panic(expected: 'ICS04: missing commitment')]
fn test_timeout_packet_with_height() {
    try_timeout_packet(HEIGHT(11), TIMESTAMP(1000));
}

#[test]
#[should_panic(expected: 'ICS04: missing commitment')]
fn test_timeout_packet_with_timestamp() {
    try_timeout_packet(HEIGHT(1000), TIMESTAMP(11));
}

#[test]
#[should_panic(expected: 'ICS04: packet not timed out')]
fn test_timeout_pending_packet() {
    try_timeout_packet(HEIGHT(1000), TIMESTAMP(1000));
}
