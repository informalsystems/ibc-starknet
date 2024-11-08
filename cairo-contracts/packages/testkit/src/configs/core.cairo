use starknet_ibc_core::channel::{ChannelOrdering, MsgChanOpenInit};
use starknet_ibc_testkit::dummies::{CONNECTION_ID, PORT_ID, CHANNEL_VERSION};

#[derive(Clone, Debug, Drop, Serde)]
pub struct CoreConfig {}

#[generate_trait]
pub impl CoreConfigImpl of CoreConfigTrait {
    fn default() -> CoreConfig {
        CoreConfig {}
    }

    fn dummy_msg_chan_open_init(self: @CoreConfig) -> MsgChanOpenInit {
        MsgChanOpenInit {
            port_id_on_a: PORT_ID(),
            conn_id_on_a: CONNECTION_ID(0),
            version_on_a: CHANNEL_VERSION(),
            port_id_on_b: PORT_ID(),
            ordering: ChannelOrdering::Unordered,
        }
    }
}
