#[starknet::contract]
pub mod ERC20Mintable {
    use openzeppelin_access::ownable::OwnableComponent;
    use openzeppelin_token::erc20::{ERC20Component, ERC20HooksEmptyImpl, interface::IERC20Metadata};
    use starknet::ContractAddress;
    use starknet::storage::{StoragePointerReadAccess, StoragePointerWriteAccess};
    use starknet_ibc_utils::mintable::ERC20MintableComponent;
    use starknet_ibc_utils::mintable::ERC20MintableComponent::ERC20MintableInternalTrait;

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);
    component!(path: ERC20MintableComponent, storage: mintable, event: MintableEvent);
    component!(path: ERC20Component, storage: erc20, event: ERC20Event);

    // Ownable Mixin
    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    // ERC20 Mintable
    #[abi(embed_v0)]
    impl ERC20MintableImpl = ERC20MintableComponent::ERC20Mintable<ContractState>;

    #[abi(embed_v0)]
    impl ERC20Impl = ERC20Component::ERC20Impl<ContractState>;
    #[abi(embed_v0)]
    impl ERC20CamelOnlyImpl = ERC20Component::ERC20CamelOnlyImpl<ContractState>;
    impl ERC20InternalImpl = ERC20Component::InternalImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        mintable: ERC20MintableComponent::Storage,
        #[substorage(v0)]
        erc20: ERC20Component::Storage,
        // The decimals value is stored locally in the contract.
        // ref: https://docs.openzeppelin.com/contracts-cairo/0.20.0/erc20#the_storage_approach
        decimals: u8,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        #[flat]
        MintableEvent: ERC20MintableComponent::Event,
        #[flat]
        ERC20Event: ERC20Component::Event,
    }

    #[constructor]
    fn constructor(
        ref self: ContractState,
        name: ByteArray,
        symbol: ByteArray,
        decimals: u8,
        initial_supply: u256,
        recipient: ContractAddress,
        owner: ContractAddress,
    ) {
        self.ownable.initializer(owner);
        self.mintable.initializer();
        self.erc20.initializer(name, symbol);
        self.erc20.mint(recipient, initial_supply);

        self._set_decimals(decimals);
    }


    #[abi(embed_v0)]
    impl ERC20MetadataImpl of IERC20Metadata<ContractState> {
        fn name(self: @ContractState) -> ByteArray {
            self.erc20.name()
        }

        fn symbol(self: @ContractState) -> ByteArray {
            self.erc20.symbol()
        }

        fn decimals(self: @ContractState) -> u8 {
            self.decimals.read()
        }
    }

    #[generate_trait]
    impl InternalImpl of InternalTrait {
        fn _set_decimals(ref self: ContractState, decimals: u8) {
            self.decimals.write(decimals);
        }
    }
}
