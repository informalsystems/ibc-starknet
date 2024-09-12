#[starknet::component]
pub mod ERC20MintableComponent {
    use core::num::traits::CheckedAdd;
    use core::num::traits::CheckedSub;
    use core::num::traits::Zero;
    use openzeppelin_token::erc20::ERC20Component::InternalTrait;
    use openzeppelin_token::erc20::ERC20Component;
    use openzeppelin_token::erc20::erc20::ERC20Component::Transfer;
    use starknet::ContractAddress;
    use starknet::get_caller_address;
    use starknet_ibc_utils::mintable::errors::MintableErrors;
    use starknet_ibc_utils::mintable::interface::IERC20Mintable;

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
            let permitted_minter = self.read_permission();
            assert(permitted_minter == get_caller_address(), MintableErrors::UNAUTHORIZED_MINTER);

            self.mint(recipient, amount);
        }

        fn permissioned_burn(
            ref self: ComponentState<TContractState>, account: ContractAddress, amount: u256
        ) {
            let permitted_burner = self.read_permission();
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
            self.write_permission(get_caller_address());
        }

        fn mint(
            ref self: ComponentState<TContractState>, recipient: ContractAddress, amount: u256
        ) {
            assert(recipient.is_non_zero(), MintableErrors::MINT_TO_ZERO);

            let new_amount = self.read_total_supply().checked_add(amount);

            assert(new_amount.is_some(), MintableErrors::OVERFLOWED_AMOUNT);

            self.write_total_supply(new_amount.unwrap());

            let new_balance = self.read_balance(recipient).checked_add(amount);

            assert(new_balance.is_some(), MintableErrors::OVERFLOWED_AMOUNT);

            self.write_balance(recipient, new_balance.unwrap());

            self.emit_transfer_event(Zero::zero(), recipient, amount);
        }

        fn burn(ref self: ComponentState<TContractState>, account: ContractAddress, amount: u256) {
            assert(account.is_non_zero(), MintableErrors::BURN_FROM_ZERO);

            let total_supply = self.read_total_supply();

            assert(total_supply >= amount, MintableErrors::INSUFFICIENT_SUPPLY);

            let new_amount = total_supply.checked_sub(amount);

            assert(new_amount.is_some(), MintableErrors::OVERFLOWED_AMOUNT);

            self.write_total_supply(new_amount.unwrap());

            let balance = self.read_balance(account);

            assert(balance >= amount, MintableErrors::INSUFFICIENT_BALANCE);

            let new_balance = balance.checked_sub(amount);

            assert(new_balance.is_some(), MintableErrors::OVERFLOWED_AMOUNT);

            self.write_balance(account, new_balance.unwrap());

            self.emit_transfer_event(account, Zero::zero(), amount);
        }
    }

    #[generate_trait]
    impl ERC20ReaderImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl ERC20: ERC20Component::HasComponent<TContractState>
    > of ERC20ReaderTrait<TContractState> {
        fn read_permission(self: @ComponentState<TContractState>) -> ContractAddress {
            self.permission.read()
        }

        fn read_balance(self: @ComponentState<TContractState>, account: ContractAddress) -> u256 {
            let erc20_comp = get_dep_component!(self, ERC20);
            erc20_comp.ERC20_balances.read(account)
        }

        fn read_total_supply(self: @ComponentState<TContractState>) -> u256 {
            let erc20_comp = get_dep_component!(self, ERC20);
            erc20_comp.ERC20_total_supply.read()
        }
    }

    #[generate_trait]
    impl ERC20WriterImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl ERC20: ERC20Component::HasComponent<TContractState>
    > of ERC20WriterTrait<TContractState> {
        fn write_permission(
            ref self: ComponentState<TContractState>, permitted_minter: ContractAddress
        ) {
            self.permission.write(permitted_minter);
        }

        fn write_balance(
            ref self: ComponentState<TContractState>, account: ContractAddress, amount: u256
        ) {
            let mut erc20_comp = get_dep_component_mut!(ref self, ERC20);
            erc20_comp.ERC20_balances.write(account, amount);
        }

        fn write_total_supply(ref self: ComponentState<TContractState>, amount: u256) {
            let mut erc20_comp = get_dep_component_mut!(ref self, ERC20);
            erc20_comp.ERC20_total_supply.write(amount);
        }
    }

    #[generate_trait]
    impl ERC20EventEmitterImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl ERC20: ERC20Component::HasComponent<TContractState>
    > of ERC20EventEmitterTrait<TContractState> {
        fn emit_transfer_event(
            ref self: ComponentState<TContractState>,
            from: ContractAddress,
            to: ContractAddress,
            value: u256
        ) {
            let mut erc20_comp = get_dep_component_mut!(ref self, ERC20);
            erc20_comp.emit(Transfer { from, to, value });
        }
    }
}
