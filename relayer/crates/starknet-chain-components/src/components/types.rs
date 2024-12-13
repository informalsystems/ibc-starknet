use cgp::prelude::*;
use hermes_cosmos_chain_components::components::client::{
    ClientIdTypeComponent, CommitmentProofTypeComponent, ConnectionEndTypeComponent,
    ConnectionIdTypeComponent,
};
use ibc::core::connection::types::ConnectionEnd;

use crate::types::client_id::ClientId;
use crate::types::connection_id::ConnectionId;

cgp_preset! {
    StarknetChainTypes {
        ClientIdTypeComponent: ClientId,
        ConnectionIdTypeComponent: ConnectionId,
        // FIXME: design proper commitment proof
        CommitmentProofTypeComponent: (),
        ConnectionEndTypeComponent: ConnectionEnd,
    }
}
