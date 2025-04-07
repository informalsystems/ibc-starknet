use std::sync::Arc;

use starknet_v13::accounts::SingleOwnerAccount;
use starknet_v13::providers::jsonrpc::HttpTransport;
use starknet_v13::providers::JsonRpcClient;
use starknet_v13::signers::LocalWallet;

pub type StarknetAccount = SingleOwnerAccount<Arc<JsonRpcClient<HttpTransport>>, LocalWallet>;
