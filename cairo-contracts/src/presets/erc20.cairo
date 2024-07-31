#[starknet::contract]
pub(crate) mod ERC20Mintable {
    use openzeppelin::access::ownable::OwnableComponent;
    use openzeppelin::token::erc20::{ERC20Component, ERC20HooksEmptyImpl};
    use starknet::{ContractAddress, ClassHash};
    use starknet_ibc::apps::mintable::component::ERC20MintableComponent::ERC20MintableInternalTrait;
    use starknet_ibc::apps::mintable::component::ERC20MintableComponent;

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

    // ERC20 Mixin
    #[abi(embed_v0)]
    impl ERC20MixinImpl = ERC20Component::ERC20MixinImpl<ContractState>;
    impl ERC20InternalImpl = ERC20Component::InternalImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
        #[substorage(v0)]
        mintable: ERC20MintableComponent::Storage,
        #[substorage(v0)]
        erc20: ERC20Component::Storage,
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
        initial_supply: u256,
        recipient: ContractAddress,
        owner: ContractAddress
    ) {
        self.ownable.initializer(owner);
        self.mintable.initializer();
        self.erc20.initializer(name, symbol);
        self.erc20.mint(recipient, initial_supply);
    }
}
