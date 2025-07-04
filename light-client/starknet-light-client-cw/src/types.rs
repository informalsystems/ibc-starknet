use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ContractResponse {
    StateRoot(String),
    ValidStorageProof,
    GlobalContractTrieRoot(String),
    ContractRoot(String),
    CorrectStorageProof,
}
