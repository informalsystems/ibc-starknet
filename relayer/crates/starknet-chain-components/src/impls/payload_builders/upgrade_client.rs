use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    ClientUpgradePayloadBuilder, ClientUpgradePayloadBuilderComponent, HasHeightType,
    HasUpgradeClientPayloadType,
};
use hermes_core::encoding_components::traits::{CanDecode, HasEncodedType, HasEncoding};
use hermes_cosmos_core::chain_components::impls::CosmosUpgradeClientPayload;
use hermes_prelude::*;
use ibc::core::client::types::Height;
use ibc::core::host::types::path::{UpgradeClientStatePath, UpgradeConsensusStatePath};
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
};

pub struct BuildStarknetUpgradeClientPayload;

#[cgp_provider(ClientUpgradePayloadBuilderComponent)]
impl<Chain, CounterParty, Encoding> ClientUpgradePayloadBuilder<Chain, CounterParty>
    for BuildStarknetUpgradeClientPayload
where
    Chain: HasHeightType<Height = u64>
        + HasUpgradeClientPayloadType<UpgradeClientPayload = CosmosUpgradeClientPayload>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + CanQueryStorageProof<StorageProof = StorageProof, StorageKey = Felt>
        + CanCallContract
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseAsyncError<Encoding::Error>,
    Encoding: Async
        + CanDecode<ViaCairo, u64>
        + CanDecode<ViaCairo, (CairoStarknetClientState, CairoStarknetConsensusState)>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn upgrade_client_payload(
        chain: &Chain,
        upgrade_height: &u64,
    ) -> Result<Chain::UpgradeClientPayload, Chain::Error> {
        let encoding = chain.encoding();

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

            let onchain_final_height: u64 = encoding.decode(&output).map_err(Chain::raise_error)?;

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

        let (client_state, consensus_state): (
            CairoStarknetClientState,
            CairoStarknetConsensusState,
        ) = encoding.decode(&output).map_err(Chain::raise_error)?;

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

        Ok(StarknetUpgradeClientPayload {
            upgrade_height: Height::new(0, *upgrade_height).unwrap(),
            client_state,
            consensus_state,
            client_state_proof,
            consensus_state_proof,
        }
        .into())
    }
}

impl From<StarknetUpgradeClientPayload> for CosmosUpgradeClientPayload {
    fn from(payload: StarknetUpgradeClientPayload) -> Self {
        // TODO(rano): transform starknet upgrade client payload to cosmos payload
        todo!()
    }
}
