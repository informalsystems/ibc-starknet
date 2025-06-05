use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct QueryBlockWithTxHashesRequest {
    pub block_id: &'static str,
}

#[derive(Debug, Deserialize)]
pub struct QueryBlockWithTxHashesResponse {
    pub block_number: u64,
    pub block_hash: String,
    pub timestamp: u64,
}
