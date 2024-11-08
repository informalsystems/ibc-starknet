use starknet_ibc_apps::transfer::VERSION;
use starknet_ibc_core::channel::{ChannelOrdering, MsgChanOpenInit, MsgChanOpenTry, MsgChanOpenAck};
use starknet_ibc_testkit::dummies::{
    HEIGHT, CONNECTION_ID, CHANNEL_ID, PORT_ID, VERSION_PROPOSAL, STATE_PROOF
};

#[derive(Clone, Debug, Drop, Serde)]
pub struct CoreConfig {
    channel_ordering: ChannelOrdering,
}

#[generate_trait]
pub impl CoreConfigImpl of CoreConfigTrait {
    fn default() -> CoreConfig {
        CoreConfig { channel_ordering: ChannelOrdering::Unordered, }
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
            chan_id_on_a: CHANNEL_ID(1),
            version_on_a: VERSION(),
            proof_chan_end_on_a: STATE_PROOF(),
            proof_height_on_a: HEIGHT(10),
            ordering: *self.channel_ordering,
        }
    }

    fn dummy_msg_chan_open_ack(self: @CoreConfig) -> MsgChanOpenAck {
        MsgChanOpenAck {
            port_id_on_a: PORT_ID(),
            chan_id_on_a: CHANNEL_ID(0),
            chan_id_on_b: CHANNEL_ID(1),
            version_on_b: VERSION(),
            proof_chan_end_on_b: STATE_PROOF(),
            proof_height_on_b: HEIGHT(10),
        }
    }
}
