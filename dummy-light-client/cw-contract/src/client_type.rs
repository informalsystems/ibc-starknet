use ibc_client_cw::api::ClientType;
use ibc_client_dummy::{ClientState, ConsensusState};

pub struct DummyLightClient;

impl<'a> ClientType<'a> for DummyLightClient {
    type ClientState = ClientState;
    type ConsensusState = ConsensusState;
}
