pub mod core;
pub mod erc20;
pub mod apps {
    mod transfer;
    pub use transfer::TransferApp;
}
pub mod clients {
    mod cometbft;
    mod mock;
    pub use cometbft::CometClient;
    pub use mock::MockClient;
}
pub mod libraries {
    mod comet;
    mod ics23;
    mod protobuf;
    pub use comet::CometLib;
    pub use ics23::Ics23Lib;
    pub use protobuf::ProtobufLib;
}
#[cfg(test)]
mod tests {
    pub(crate) mod channel;
    pub(crate) mod client;
    pub(crate) mod connection;
    pub(crate) mod erc20;
    pub(crate) mod transfer;
}
