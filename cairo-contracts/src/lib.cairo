pub mod core;
pub mod erc20;

pub mod apps {
    pub mod transfer;
}
pub mod clients {
    pub mod cometbft;
}
mod tests {
    #[cfg(test)]
    mod constants;
    #[cfg(test)]
    mod test_client;
    #[cfg(test)]
    mod test_transfer;
    #[cfg(test)]
    mod configs {
        pub(crate) mod comet;
        pub(crate) mod transfer;
    }
    mod mocks {
        mod transfer_mock;
    }
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
