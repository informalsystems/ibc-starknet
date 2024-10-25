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

    /// Byte representation of the standard success acknowledgment JSON string
    /// `{"result":"AQ=="}`. This serves as a workaround for the lack of JSON
    /// serialization in Cairo, offering a more cost-effective way to confirm
    /// successful packet transmission. Any other value indicates failure.
    pub fn SUCCESS_ACK() -> starknet_ibc_core::channel::Acknowledgement {
        array![123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 34, 65, 81, 61, 61, 34, 125].into()
    }
}

#[cfg(test)]
mod tests {
    mod ack;
    mod transfer;
}
