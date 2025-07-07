use hermes_core::chain_components::traits::{
    ChannelEndTypeComponent, ChannelIdTypeComponent, ClientIdTypeComponent, ClientStatus,
    ClientStatusTypeComponent, ConnectionEndTypeComponent, ConnectionIdTypeComponent,
};
use hermes_prelude::*;

use crate::types::{ChannelEnd, ChannelId, ClientId, ConnectionEnd, ConnectionId};

pub struct StarknetChainTypes;

delegate_components! {
    StarknetChainTypes {
        ClientIdTypeComponent: ClientId,
        ConnectionIdTypeComponent: ConnectionId,
        ChannelIdTypeComponent: ChannelId,
        ConnectionEndTypeComponent: ConnectionEnd,
        ChannelEndTypeComponent: ChannelEnd,
        ClientStatusTypeComponent: ClientStatus,
    }
}
