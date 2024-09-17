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
mod tests {
    #[cfg(test)]
    mod test_channel;
    #[cfg(test)]
    mod test_client;
    #[cfg(test)]
    mod test_transfer;

    pub use handles::app::{AppContract, AppHandleImpl, AppHandle};
    pub use handles::client::{ClientHandleImpl, ClientHandle};
    pub use handles::core::{CoreContract, CoreHandleImpl, CoreHandle};
    pub use handles::erc20::{ERC20HandleImpl, ERC20Handle};

    mod handles {
        pub mod app;
        pub mod client;
        pub mod core;
        pub mod erc20;
    }
}
