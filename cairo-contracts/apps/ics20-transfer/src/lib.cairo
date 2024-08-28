mod component;
mod erc20_call;
mod errors;
mod interface;
pub mod transferrable;
pub mod types;

pub use component::ICS20TransferComponent;
pub use erc20_call::{ERC20Contract, ERC20ContractTrait};
pub use errors::TransferErrors;
pub use interface::{
    ISendTransfer, ISendTransferDispatcher, ISendTransferDispatcherTrait, IRecvPacket,
    IRecvPacketDispatcher, IRecvPacketDispatcherTrait, ITokenAddress, ITokenAddressDispatcher,
    ITokenAddressDispatcherTrait
};

/// The poseidon hash of the transfer port id.
pub const TRANSFER_PORT_ID_HASH: felt252 =
    506076466176013583354797631368330115868609515147080483618120063858966368900;

pub fn TRANSFER_PORT_ID() -> ByteArray {
    "transfer"
}
