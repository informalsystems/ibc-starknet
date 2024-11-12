use starknet_ibc_core::host::{ClientId, ConnectionId, PathPrefix};

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct Counterparty {
    pub client_id: ClientId,
    pub connection_id: ConnectionId,
    pub prefix: PathPrefix,
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct Version {
    pub identifier: ByteArray,
    pub features: Array<ByteArray>,
}
