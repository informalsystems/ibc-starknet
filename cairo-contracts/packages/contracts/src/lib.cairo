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
#[cfg(test)]
mod tests {
    pub(crate) mod channel;
    pub(crate) mod client;
    pub(crate) mod connection;
    pub(crate) mod erc20;
    pub(crate) mod transfer;
}
