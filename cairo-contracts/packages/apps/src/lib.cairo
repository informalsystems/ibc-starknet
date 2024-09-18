pub mod transfer {
    mod erc20_call;
    mod errors;
    pub mod types;

    pub use components::transfer::TokenTransferComponent;
    pub use components::transferrable::TransferrableComponent;
    pub use erc20_call::{ERC20Contract, ERC20ContractTrait};
    pub use errors::TransferErrors;
    pub use interfaces::transfer::{
        ISendTransfer, ISendTransferDispatcher, ISendTransferDispatcherTrait, ITokenAddress,
        ITokenAddressDispatcher, ITokenAddressDispatcherTrait,
    };
    pub use interfaces::transferrable::{
        ITransferrable, ITransferrableDispatcher, ITransferrableDispatcherTrait
    };
    pub mod components {
        pub mod transfer;
        pub mod transferrable;
    }
    pub mod interfaces {
        pub mod transfer;
        pub mod transferrable;
    }

    /// The poseidon hash of the transfer port id.
    pub const TRANSFER_PORT_ID_HASH: felt252 =
        506076466176013583354797631368330115868609515147080483618120063858966368900;

    pub fn TRANSFER_PORT_ID() -> ByteArray {
        "transfer"
    }
}
pub mod tests {
    mod config;
    mod dummy;
    mod extend_spy;
    #[cfg(test)]
    mod test_transfer;
    #[cfg(test)]
    pub use mocks::mock_transfer::MockTransferApp;
    pub use config::{TransferAppConfig, TransferAppConfigImpl, TransferAppConfigTrait};
    pub use dummy::{
        NAME, SYMBOL, PUBKEY, AMOUNT, SUPPLY, OWNER, STARKNET, COSMOS, SALT, DECIMALS, CLASS_HASH,
        EMPTY_MEMO
    };

    pub use extend_spy::{TransferEventSpyExtImpl, TransferEventSpyExt};
    #[cfg(test)]
    mod mocks {
        pub mod mock_transfer;
    }
}
