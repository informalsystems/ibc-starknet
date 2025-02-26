use openzeppelin_testing::declare_and_deploy;
use openzeppelin_testing::events::EventSpyExtImpl;
use openzeppelin_utils::serde::SerializedAppend;
use snforge_std::ContractClass;
use starknet::ContractAddress;
use starknet_ibc_apps::transfer::types::{MsgTransfer, PrefixedDenom};
use starknet_ibc_apps::transfer::{
    ICreateIbcTokenDispatcher, ICreateIbcTokenDispatcherTrait, ISendTransferDispatcher,
    ISendTransferDispatcherTrait, ITransferQueryDispatcher, ITransferQueryDispatcherTrait,
};
use starknet_ibc_core::channel::IAppCallbackDispatcher;
use starknet_ibc_core::router::AppContract;

#[generate_trait]
pub impl AppHandleImpl of AppHandle {
    fn deploy_transfer(
        contract_name: ByteArray, owner: ContractAddress, erc20_class: ContractClass,
    ) -> AppContract {
        let mut call_data = array![];

        call_data.append_serde(owner);
        call_data.append_serde(erc20_class.class_hash);

        let address = declare_and_deploy(contract_name, call_data);

        AppContract { address }
    }

    fn send_dispatcher(self: @AppContract) -> ISendTransferDispatcher {
        ISendTransferDispatcher { contract_address: *self.address }
    }

    fn callback_dispatcher(self: @AppContract) -> IAppCallbackDispatcher {
        IAppCallbackDispatcher { contract_address: *self.address }
    }

    fn ibc_token_address(self: @AppContract, token_key: felt252) -> ContractAddress {
        ITransferQueryDispatcher { contract_address: *self.address }.ibc_token_address(token_key)
    }

    fn send_transfer(self: @AppContract, msg: MsgTransfer) {
        self.send_dispatcher().send_transfer(msg);
    }

    fn create_ibc_token(self: @AppContract, denom: PrefixedDenom) -> ContractAddress {
        ICreateIbcTokenDispatcher { contract_address: *self.address }.create_ibc_token(denom)
    }
}
