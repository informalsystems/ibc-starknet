#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_cosmos_chain_components::components::client::{
        ChannelEndTypeComponent, ChannelIdTypeComponent, ClientIdTypeComponent,
        ConnectionEndTypeComponent, ConnectionIdTypeComponent,
    };

    use crate::types::channel_id::{ChannelEnd, ChannelId};
    use crate::types::client_id::ClientId;
    use crate::types::connection_id::{ConnectionEnd, ConnectionId};

    cgp_preset! {
        StarknetChainTypes {
            ClientIdTypeComponent: ClientId,
            ConnectionIdTypeComponent: ConnectionId,
            ChannelIdTypeComponent: ChannelId,
            ConnectionEndTypeComponent: ConnectionEnd,
            ChannelEndTypeComponent: ChannelEnd,
        }
    }
}
