use starknet_ibc_apps::transfer::VERSION;
use starknet_ibc_core::channel::{
    ChannelOrdering, MsgChanOpenInit, MsgChanOpenTry, MsgChanOpenAck, MsgChanOpenConfirm
};
use starknet_ibc_core::connection::{
    MsgConnOpenInit, MsgConnOpenTry, MsgConnOpenAck, MsgConnOpenConfirm, VersionImpl
};
use starknet_ibc_testkit::dummies::{
    HEIGHT, CONNECTION_ID, CHANNEL_ID, PORT_ID, VERSION_PROPOSAL, STATE_PROOF, CLIENT_ID, IBC_PREFIX
};
use starknet_ibc_testkit::handles::{CoreContract, CoreHandle};

#[derive(Clone, Debug, Drop, Serde)]
pub struct CoreConfig {
    chan_sequence_on_a: u64,
    chan_sequence_on_b: u64,
    channel_ordering: ChannelOrdering,
}

#[generate_trait]
pub impl CoreConfigImpl of CoreConfigTrait {
    fn default() -> CoreConfig {
        CoreConfig {
            chan_sequence_on_a: 0,
            chan_sequence_on_b: 0,
            channel_ordering: ChannelOrdering::Unordered
        }
    }

    fn set_chan_sequence_on_b(ref self: CoreConfig, sequence: u64) {
        self.chan_sequence_on_b = sequence;
    }

    fn dummy_msg_conn_open_init(self: @CoreConfig) -> MsgConnOpenInit {
        MsgConnOpenInit {
            client_id_on_a: CLIENT_ID(),
            client_id_on_b: CLIENT_ID(),
            prefix_on_b: IBC_PREFIX(),
            version: VersionImpl::supported(),
            delay_period: 0,
        }
    }

    fn dummy_msg_conn_open_try(self: @CoreConfig) -> MsgConnOpenTry {
        MsgConnOpenTry {
            client_id_on_b: CLIENT_ID(),
            client_id_on_a: CLIENT_ID(),
            conn_id_on_a: CONNECTION_ID(0),
            prefix_on_a: IBC_PREFIX(),
            version_on_a: VersionImpl::supported(),
            proof_conn_end_on_a: STATE_PROOF(),
            proof_height_on_a: HEIGHT(10),
            delay_period: 0,
        }
    }

    fn dummy_msg_conn_open_ack(self: @CoreConfig) -> MsgConnOpenAck {
        MsgConnOpenAck {
            conn_id_on_a: CONNECTION_ID(0),
            conn_id_on_b: CONNECTION_ID(0),
            proof_conn_end_on_b: STATE_PROOF(),
            proof_height_on_b: HEIGHT(10),
            version: VersionImpl::supported(),
        }
    }

    fn dummy_msg_conn_open_confirm(self: @CoreConfig) -> MsgConnOpenConfirm {
        MsgConnOpenConfirm {}
    }

    fn dummy_msg_chan_open_init(self: @CoreConfig) -> MsgChanOpenInit {
        MsgChanOpenInit {
            port_id_on_a: PORT_ID(),
            conn_id_on_a: CONNECTION_ID(0),
            port_id_on_b: PORT_ID(),
            version_proposal: VERSION_PROPOSAL(),
            ordering: *self.channel_ordering,
        }
    }

    fn dummy_msg_chan_open_try(self: @CoreConfig) -> MsgChanOpenTry {
        MsgChanOpenTry {
            port_id_on_b: PORT_ID(),
            conn_id_on_b: CONNECTION_ID(0),
            port_id_on_a: PORT_ID(),
            chan_id_on_a: CHANNEL_ID(*self.chan_sequence_on_b),
            version_on_a: VERSION(),
            proof_chan_end_on_a: STATE_PROOF(),
            proof_height_on_a: HEIGHT(10),
            ordering: *self.channel_ordering,
        }
    }

    fn dummy_msg_chan_open_ack(self: @CoreConfig) -> MsgChanOpenAck {
        MsgChanOpenAck {
            port_id_on_a: PORT_ID(),
            chan_id_on_a: CHANNEL_ID(*self.chan_sequence_on_a),
            chan_id_on_b: CHANNEL_ID(*self.chan_sequence_on_b),
            version_on_b: VERSION(),
            proof_chan_end_on_b: STATE_PROOF(),
            proof_height_on_b: HEIGHT(10),
        }
    }

    fn dummy_msg_chan_open_confirm(self: @CoreConfig) -> MsgChanOpenConfirm {
        MsgChanOpenConfirm {
            port_id_on_b: PORT_ID(),
            chan_id_on_b: CHANNEL_ID(*self.chan_sequence_on_b),
            proof_chan_end_on_a: STATE_PROOF(),
            proof_height_on_a: HEIGHT(10),
        }
    }

    fn create_channel(ref self: CoreConfig, core: @CoreContract) {
        core.chan_open_init(self.dummy_msg_chan_open_init());

        core.chan_open_ack(self.dummy_msg_chan_open_ack());

        self.chan_sequence_on_a += 1;
    }
}
