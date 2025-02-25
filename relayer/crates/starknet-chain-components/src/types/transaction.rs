use starknet::accounts::Call;

pub struct StarknetTransaction {
    pub calls: Vec<Call>,
}
