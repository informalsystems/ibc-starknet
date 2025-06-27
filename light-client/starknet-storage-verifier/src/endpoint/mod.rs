use starknet::providers::ProviderError;
use starknet::providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, Url};
use starknet_core::types::{ConfirmedBlockId, Felt};
use starknet_core::types::{ContractStorageKeys, StorageProof};

const SEPOLIA_RPC_ENDPOINT: &str = "https://starknet-sepolia.reddio.com/rpc/v0_8";
const MAINNET_RPC_ENDPOINT: &str = "https://starknet-mainnet.reddio.com/rpc/v0_8";

pub struct Endpoint(pub JsonRpcClient<HttpTransport>);

impl Endpoint {
    pub fn new(endpoint: impl AsRef<str>) -> Self {
        Self(JsonRpcClient::new(HttpTransport::new(
            Url::parse(endpoint.as_ref()).unwrap(),
        )))
    }

    pub fn sepolia() -> Self {
        Self::new(SEPOLIA_RPC_ENDPOINT)
    }

    pub fn mainnet() -> Self {
        Self::new(MAINNET_RPC_ENDPOINT)
    }

    pub async fn get_contract_proof(
        &self,
        contract_addresses: &[Felt],
        block_id: ConfirmedBlockId,
    ) -> Result<StorageProof, ProviderError> {
        self.0
            .get_storage_proof(block_id, [], contract_addresses, [])
            .await
    }

    pub async fn get_class_proof(
        &self,
        class_hashes: &[Felt],
        block_id: ConfirmedBlockId,
    ) -> Result<StorageProof, ProviderError> {
        self.0
            .get_storage_proof(block_id, class_hashes, [], [])
            .await
    }

    pub async fn get_membership_proof(
        &self,
        contracts_storage_keys: &[ContractStorageKeys],
        block_id: ConfirmedBlockId,
    ) -> Result<StorageProof, ProviderError> {
        self.0
            .get_storage_proof(block_id, [], [], contracts_storage_keys)
            .await
    }
}
