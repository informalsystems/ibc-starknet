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

    pub mod handles {
        mod app;
        mod client;
        mod core;
        mod erc20;

        pub(crate) use app::{AppContract, AppHandleImpl, AppHandle};
        pub(crate) use client::{ClientHandleImpl, ClientHandle};
        pub(crate) use core::{CoreContract, CoreHandleImpl, CoreHandle};
        pub(crate) use erc20::{ERC20HandleImpl, ERC20Handle};
    }
}
