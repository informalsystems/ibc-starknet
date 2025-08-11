use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    ClientUpgradePayloadBuilder, ClientUpgradePayloadBuilderComponent, HasClientStateType,
    HasConsensusStateType, HasHeightType, HasUpgradeClientPayloadType,
};
use hermes_core::encoding_components::traits::{
    CanConvert, CanDecode, HasDefaultEncoding, HasEncodedType, HasEncoding,
};
use hermes_core::encoding_components::types::AsBytes;
use hermes_cosmos_core::chain_components::impls::CosmosUpgradeClientPayload;
use hermes_prelude::*;
use ibc::core::channel::types::channel::Counterparty;
use ibc::core::client::types::Height;
use ibc::core::host::types::path::{UpgradeClientStatePath, UpgradeConsensusStatePath};
use ibc_proto::google::protobuf::Any;
use starknet::core::types::Felt;
use starknet::macros::selector;
use starknet_crypto_lib::StarknetCryptoLib;
use starknet_storage_verifier::ibc::ibc_path_to_storage_key;
use starknet_v14::core::types::StorageProof;

use crate::traits::{
    CanCallContract, CanQueryContractAddress, CanQueryStorageProof, HasBlobType, HasSelectorType,
};
use crate::types::{
    CairoStarknetClientState, CairoStarknetConsensusState, StarknetUpgradeClientPayload,
    WasmStarknetClientState, WasmStarknetConsensusState,
};

pub struct BuildStarknetUpgradeClientPayload;

#[cgp_provider(ClientUpgradePayloadBuilderComponent)]
impl<Chain, CounterParty, CairoEncoding, ProtoEncoding>
    ClientUpgradePayloadBuilder<Chain, CounterParty> for BuildStarknetUpgradeClientPayload
where
    Chain: HasHeightType<Height = u64>
        + HasUpgradeClientPayloadType<UpgradeClientPayload = CosmosUpgradeClientPayload>
        + HasClientStateType<Chain, ClientState = WasmStarknetClientState>
        + HasConsensusStateType<Chain, ConsensusState = WasmStarknetConsensusState>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanQueryStorageProof<StorageProof = StorageProof, StorageKey = Felt>
        + CanCallContract
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + HasDefaultEncoding<AsBytes, Encoding = ProtoEncoding>
        + CanRaiseAsyncError<ProtoEncoding::Error>
        + CanRaiseAsyncError<CairoEncoding::Error>,
    CairoEncoding: Async
        + CanDecode<ViaCairo, u64>
        + CanDecode<ViaCairo, (CairoStarknetClientState, CairoStarknetConsensusState)>
        + HasEncodedType<Encoded = Vec<Felt>>,
    ProtoEncoding:
        Async + CanConvert<Chain::ClientState, Any> + CanConvert<Chain::ConsensusState, Any>,
{
    async fn upgrade_client_payload(
        chain: &Chain,
        upgrade_height: &u64,
    ) -> Result<CosmosUpgradeClientPayload, Chain::Error> {
        let cairo_encoding = chain.encoding();
        let proto_encoding = Chain::default_encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        {
            let output = chain
                .call_contract(
                    &contract_address,
                    &selector!("get_final_height"),
                    &vec![],
                    None,
                )
                .await?;

            let onchain_final_height: u64 =
                cairo_encoding.decode(&output).map_err(Chain::raise_error)?;

            assert_eq!(onchain_final_height, *upgrade_height);
        }

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("get_scheduled_upgrade"),
                &vec![],
                None,
            )
            .await?;

        let (cairo_client_state, cairo_consensus_state): (
            CairoStarknetClientState,
            CairoStarknetConsensusState,
        ) = cairo_encoding.decode(&output).map_err(Chain::raise_error)?;

        // TODO(rano): implement these two From
        let starknet_client_state = cairo_client_state.into();
        let starknet_consensus_state = cairo_consensus_state.into();

        let wasm_client_state = WasmStarknetClientState {
            client_state: starknet_client_state,
            // pass correct wasm code hash
            wasm_code_hash: vec![],
        };

        let wasm_consensus_state = WasmStarknetConsensusState {
            consensus_state: starknet_consensus_state,
        };

        let client_state = proto_encoding
            .convert(&wasm_client_state)
            .map_err(Chain::raise_error)?;

        let consensus_state = proto_encoding
            .convert(&wasm_consensus_state)
            .map_err(Chain::raise_error)?;

        let client_state_proof: StorageProof = {
            let ibc_path = UpgradeClientStatePath::new_with_default_path(*upgrade_height);

            let felt_path: Felt = ibc_path_to_storage_key(&StarknetCryptoLib, ibc_path.into());

            chain
                .query_storage_proof(upgrade_height, &contract_address, &[felt_path])
                .await?
        };

        let consensus_state_proof: StorageProof = {
            let ibc_path = UpgradeConsensusStatePath::new_with_default_path(*upgrade_height);

            let felt_path: Felt = ibc_path_to_storage_key(&StarknetCryptoLib, ibc_path.into());

            chain
                .query_storage_proof(upgrade_height, &contract_address, &[felt_path])
                .await?
        };

        Ok(CosmosUpgradeClientPayload {
            upgrade_height: Height::new(0, *upgrade_height).unwrap(),
            upgrade_client_state: client_state,
            upgrade_consensus_state: consensus_state,
            upgrade_client_state_proof: serde_json::to_vec(&client_state_proof)
                .expect("Failed to serialize client state proof"),
            upgrade_consensus_state_proof: serde_json::to_vec(&consensus_state_proof)
                .expect("Failed to serialize consensus state proof"),
        })
    }
}
