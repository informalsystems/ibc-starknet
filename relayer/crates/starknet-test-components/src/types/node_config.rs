use std::net::IpAddr;

#[derive(Clone)]
pub struct StarknetNodeConfig {
    pub rpc_addr: IpAddr,
    pub rpc_port: u16,
}
