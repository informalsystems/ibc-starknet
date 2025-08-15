use core::marker::PhantomData;

use hermes_core::chain_components::traits::{
    CanQueryClientStateWithLatestHeight, ClientUpgrade, ClientUpgradeComponent, HasClientStateType,
    HasConsensusStateType, HasIbcChainTypes, HasMessageType, HasUpgradeClientPayloadType,
};
use hermes_core::encoding_components::traits::{CanConvert, HasDefaultEncoding};
use hermes_core::encoding_components::types::AsBytes;
use hermes_core::relayer_components::transaction::traits::HasDefaultSigner;
use hermes_cosmos_core::chain_components::impls::MsgUpgradeClientProposal;
use hermes_cosmos_core::chain_components::traits::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_core::chain_components::types::Secp256k1KeyPair;
use hermes_prelude::*;
use ibc::core::host::types::identifiers::ClientId;
use ibc::primitives::proto::Any as IbcAny;
use prost_types::Any;

use crate::types::{
    StarknetUpgradeClientPayload, WasmStarknetClientState, WasmStarknetConsensusState,
};

pub struct BuildStarknetUpgradeClientMessage;

#[cgp_provider(ClientUpgradeComponent)]
impl<Chain, Counterparty, Encoding> ClientUpgrade<Chain, Counterparty>
    for BuildStarknetUpgradeClientMessage
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + CanQueryClientStateWithLatestHeight<Counterparty>
        + HasMessageType<Message = CosmosMessage>
        + HasDefaultSigner<Signer = Secp256k1KeyPair>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<&'static str>,
    Counterparty: HasUpgradeClientPayloadType<UpgradeClientPayload = StarknetUpgradeClientPayload>
        + HasClientStateType<Chain, ClientState = WasmStarknetClientState>
        + HasConsensusStateType<Chain, ConsensusState = WasmStarknetConsensusState>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: Async
        + CanConvert<Counterparty::ClientState, Any>
        + CanConvert<Counterparty::ConsensusState, Any>,
{
    async fn upgrade_client_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        payload: &StarknetUpgradeClientPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let encoding = Counterparty::default_encoding();

        let StarknetUpgradeClientPayload {
            upgrade_height,
            upgrade_client_state,
            upgrade_consensus_state,
            upgrade_client_state_proof,
            upgrade_consensus_state_proof,
        } = payload;

        let latest_client_state = chain
            .query_client_state_with_latest_height(PhantomData::<Counterparty>, client_id)
            .await?;

        let client_state = WasmStarknetClientState {
            client_state: upgrade_client_state.clone(),
            wasm_code_hash: latest_client_state.wasm_code_hash,
        };

        let consensus_state = WasmStarknetConsensusState {
            consensus_state: upgrade_consensus_state.clone(),
        };

        let upgrade_client_state = encoding
            .convert(&client_state)
            .map_err(Chain::raise_error)?;

        let upgrade_consensus_state = encoding
            .convert(&consensus_state)
            .map_err(Chain::raise_error)?;

        let client_state_proof_bytes = serde_json::to_vec(&upgrade_client_state_proof)
            .map_err(|_| "failed to serialize upgrade client state proof")
            .map_err(Chain::raise_error)?;

        let consensus_state_proof_bytes = serde_json::to_vec(&upgrade_consensus_state_proof)
            .map_err(|_| "failed to serialize upgrade consensus state proof")
            .map_err(Chain::raise_error)?;

        let upgrade_client_state_any = IbcAny {
            type_url: upgrade_client_state.type_url,
            value: upgrade_client_state.value,
        };

        let upgrade_consensus_state_any = IbcAny {
            type_url: upgrade_consensus_state.type_url,
            value: upgrade_consensus_state.value,
        };

        let upgrade_client_message = MsgUpgradeClientProposal {
            client_id: client_id.to_string(),
            client_state: Some(upgrade_client_state_any),
            consensus_state: Some(upgrade_consensus_state_any),
            proof_upgrade_client: client_state_proof_bytes,
            proof_upgrade_consensus_state: consensus_state_proof_bytes,
            signer: chain.get_default_signer().account().into(),
        };

        Ok(upgrade_client_message.to_cosmos_message())
    }
}
