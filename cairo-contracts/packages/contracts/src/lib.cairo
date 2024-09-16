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
    mod test_client;
    #[cfg(test)]
    mod test_transfer;
    #[cfg(test)]
    #[cfg(test)]
    mod setups {
        mod comet;
        mod erc20;
        mod ibc;
        mod transfer;

        pub(crate) use comet::{CometClientHandle, CometClientHandleImpl, CometClientHandleTrait};
        pub(crate) use erc20::{ERC20ContractImpl, ERC20ContractTrait};
        pub(crate) use ibc::{IBCCoreHandle, IBCCoreHandleImpl, IBCCoreHandleTrait};
        pub(crate) use transfer::{TransferAppHandleImpl, TransferAppHandleTrait};
    }
}
