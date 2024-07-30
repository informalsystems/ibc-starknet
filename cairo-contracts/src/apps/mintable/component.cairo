#[starknet::component]
pub mod ERC20MintableComponent {
    use core::num::traits::Zero;
    use openzeppelin::token::erc20::ERC20Component::InternalTrait;
    use openzeppelin::token::erc20::ERC20Component;
    use openzeppelin::token::erc20::erc20::ERC20Component::Transfer;
    use starknet::ContractAddress;
    use starknet::get_caller_address;
    use starknet_ibc::apps::mintable::errors::MintableErrors;
    use starknet_ibc::apps::mintable::interface::IERC20Mintable;

    #[storage]
    struct Storage {
        permission: ContractAddress,
    }

    #[event]
    #[derive(Drop, Debug, starknet::Event)]
    pub enum Event {}

    #[embeddable_as(ERC20Mintable)]
    pub impl ERC20MintableImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl ERC20: ERC20Component::HasComponent<TContractState>,
    > of IERC20Mintable<ComponentState<TContractState>> {
        fn permissioned_mint(
            ref self: ComponentState<TContractState>, recipient: ContractAddress, amount: u256
        ) {
            let permitted_minter = self.permission.read();
            assert(permitted_minter == get_caller_address(), MintableErrors::UNAUTHORIZED_MINTER);

            self.mint(recipient, amount);
        }

        fn permissioned_burn(
            ref self: ComponentState<TContractState>, account: ContractAddress, amount: u256
        ) {
            let permitted_burner = self.permission.read();
            assert(permitted_burner == get_caller_address(), MintableErrors::UNAUTHORIZED_BURNER);
            self.burn(account, amount);
        }
    }

    #[generate_trait]
    pub impl ERC20MintableInternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl ERC20: ERC20Component::HasComponent<TContractState>
    > of ERC20MintableInternalTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>) {
            self.permission.write(get_caller_address());
        }

        fn mint(
            ref self: ComponentState<TContractState>, recipient: ContractAddress, amount: u256
        ) {
            let mut erc20_comp = get_dep_component_mut!(ref self, ERC20);
            assert(recipient.is_non_zero(), MintableErrors::MINT_TO_ZERO);

            erc20_comp.ERC20_total_supply.write(erc20_comp.ERC20_total_supply.read() + amount);
            erc20_comp
                .ERC20_balances
                .write(recipient, erc20_comp.ERC20_balances.read(recipient) + amount);

            erc20_comp.emit(Transfer { from: Zero::zero(), to: recipient, value: amount });
        }

        fn burn(ref self: ComponentState<TContractState>, account: ContractAddress, amount: u256) {
            let mut erc20_comp = get_dep_component_mut!(ref self, ERC20);
            assert(account.is_non_zero(), MintableErrors::BURN_FROM_ZERO);

            let total_supply = erc20_comp.ERC20_total_supply.read();
            assert(total_supply >= amount, MintableErrors::INSUFFICIENT_SUPPLY);
            erc20_comp.ERC20_total_supply.write(total_supply - amount);

            let balance = erc20_comp.ERC20_balances.read(account);
            assert(balance >= amount, MintableErrors::INSUFFICIENT_BALANCE);
            erc20_comp.ERC20_balances.write(account, balance - amount);

            erc20_comp.emit(Transfer { from: account, to: Zero::zero(), value: amount });
        }
    }
}
