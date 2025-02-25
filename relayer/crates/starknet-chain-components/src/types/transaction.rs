use starknet::accounts::Call;

pub struct StarknetTransaction<Account> {
    pub calls: Vec<Call>,
    pub account: Account,
}
