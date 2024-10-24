pub mod core;
pub mod erc20;
pub mod apps {
    mod transfer;
    pub use transfer::TransferApp;
}
pub mod clients {
    mod cometbft;
    pub use cometbft::CometClient;
}
#[cfg(test)]
mod tests {
    mod channel;
    mod client;
    mod transfer;
}
