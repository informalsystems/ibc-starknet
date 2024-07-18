use ibc_client_cw::api::ClientType;
use ibc_client_starknet::{ClientState, ConsensusState};

pub struct StarknetClient;

impl<'a> ClientType<'a> for StarknetClient {
    type ClientState = ClientState;
    type ConsensusState = ConsensusState;
}
