use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::impls::CanWaitChainReachHeight;
use hermes_core::chain_components::traits::{
    CanBuildClientUpgradePayload, CanBuildUpdateClientMessage, CanBuildUpdateClientPayload,
    CanQueryChainHeight, CanQueryChainStatus, CanQueryClientStateWithLatestHeight,
    CanQueryConsensusStateWithLatestHeight, CanSendMessages, CanUpgradeClient,
    HasClientStateFields, HasClientStateType, HasConsensusStateType,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelInfo;
use hermes_core::relayer_components::transaction::impls::CanSendSingleMessageWithSigner;
use hermes_core::relayer_components::transaction::traits::HasDefaultSigner;
use hermes_core::test_components::chain_driver::traits::{
    HasChain, HasSetupUpgradeClientTestResultType,
};
use hermes_core::test_components::test_case::traits::node::{CanHaltFullNode, CanResumeFullNode};
use hermes_core::test_components::test_case::traits::upgrade_client::{
    SetupUpgradeClientTestHandler, SetupUpgradeClientTestHandlerComponent, UpgradeClientHandler,
    UpgradeClientHandlerComponent,
};
use hermes_prelude::*;
use hermes_starknet_chain_components::impls::{StarknetAddress, StarknetMessage};
use hermes_starknet_chain_components::traits::{
    CanCallContract, CanQueryContractAddress, HasBlobType, HasFeederGatewayUrl, HasSelectorType,
};
use hermes_starknet_chain_components::types::{
    CairoStarknetClientState, CairoStarknetConsensusState, Height, StarknetChainStatus,
    WasmStarknetClientState, WasmStarknetConsensusState,
};
use hermes_starknet_test_components::impls::StarknetProposalSetupClientUpgradeResult;
use hermes_starknet_test_components::types::StarknetNodeConfig;
use ibc::core::client::types::error::ClientError;
use ibc::core::host::types::identifiers::ChainId;
use ibc::primitives::Timestamp;
use starknet::core::types::Felt;
use starknet::macros::selector;
use starknet_block_verifier::Endpoint as FeederGatewayEndpoint;
use starknet_crypto::get_public_key;

use crate::contexts::HasChainNodeConfig;
use crate::impls::StarknetFullNodeResumeOptions;

pub struct StarknetHandleUpgradeClient;

#[cgp_provider(UpgradeClientHandlerComponent)]
impl<ChainDriverA, ChainDriverB, ChainA, ChainB, Encoding>
    UpgradeClientHandler<ChainDriverA, ChainDriverB> for StarknetHandleUpgradeClient
where
    ChainDriverA: HasChain<Chain = ChainA>
        + HasChainNodeConfig<ChainNodeConfig = StarknetNodeConfig>
        + HasSetupUpgradeClientTestResultType<
            SetupUpgradeClientTestResult = StarknetProposalSetupClientUpgradeResult,
        > + CanHaltFullNode
        + CanResumeFullNode<ResumeFullNodeOptions = StarknetFullNodeResumeOptions>
        + CanRaiseAsyncError<ChainA::Error>
        + CanRaiseAsyncError<ChainB::Error>
        + CanRaiseAsyncError<ClientError>
        + CanRaiseAsyncError<Encoding::Error>,
    ChainDriverB: HasChain<Chain = ChainB>,
    ChainA: CanBuildClientUpgradePayload<ChainB>
        + CanWaitChainReachHeight
        + CanQueryChainHeight<Height = u64>
        + HasFeederGatewayUrl
        + CanBuildUpdateClientPayload<ChainB>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address"), Address = StarknetAddress>
        + CanCallContract
        + HasDefaultSigner
        + CanLog<LevelInfo>
        + CanSendSingleMessageWithSigner<Message = StarknetMessage>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + HasClientStateType<ChainB>
        + HasClientStateFields<ChainB, ChainId = ChainId>
        + HasAsyncErrorType,
    ChainB: CanUpgradeClient<ChainA>
        + HasDefaultSigner
        + CanSendMessages
        + CanSendSingleMessageWithSigner
        + CanQueryClientStateWithLatestHeight<ChainA>
        + CanBuildUpdateClientMessage<ChainA>,
    Encoding: CanDecode<ViaCairo, u64> + HasEncodedType<Encoded = Vec<Felt>> + HasAsyncErrorType,
{
    async fn handle_upgrade_client(
        chain_driver_a: &ChainDriverA,
        setup_result: &StarknetProposalSetupClientUpgradeResult,
        chain_driver_b: &ChainDriverB,
        client_id_b: &ChainB::ClientId,
    ) -> Result<(), ChainDriverA::Error> {
        let chain_a = chain_driver_a.chain();

        let chain_b = chain_driver_b.chain();

        let encoding = chain_a.encoding();

        let ibc_core_contract_address = chain_a
            .query_contract_address(PhantomData)
            .await
            .map_err(ChainDriverA::raise_error)?;

        let output = chain_a
            .call_contract(
                &ibc_core_contract_address,
                &selector!("get_final_height"),
                &vec![],
                None,
            )
            .await
            .map_err(ChainDriverA::raise_error)?;

        let onchain_final_height: u64 = encoding
            .decode(&output)
            .map_err(ChainDriverA::raise_error)?;

        chain_a
            .log(
                &format!("Waiting till Starknet final height: {onchain_final_height}"),
                &LevelInfo,
            )
            .await;

        chain_a
            // Starknet chain backup interval is 10.
            // Making sure (waiting 2 intervals) final_height block is present in restarted chain.
            .wait_chain_reach_height(&(onchain_final_height + 20))
            .await
            .map_err(ChainDriverA::raise_error)?;

        chain_a
            .log(
                &format!("Reached Starknet final height: {onchain_final_height}"),
                &LevelInfo,
            )
            .await;

        // Must update client till final height
        let client_b_state = chain_b
            .query_client_state_with_latest_height(PhantomData, client_id_b)
            .await
            .map_err(ChainDriverA::raise_error)?;

        let client_b_state_height = ChainA::client_state_latest_height(&client_b_state);

        let client_b_update_payload = chain_a
            .build_update_client_payload(
                &client_b_state_height,
                &onchain_final_height,
                client_b_state,
            )
            .await
            .map_err(ChainDriverA::raise_error)?;

        let update_messages = chain_b
            .build_update_client_message(client_id_b, client_b_update_payload)
            .await
            .map_err(ChainDriverA::raise_error)?;

        chain_b
            .send_messages(update_messages)
            .await
            .map_err(ChainDriverA::raise_error)?;

        chain_a
            .log(
                &format!("Updated Starknet client till final height: {onchain_final_height}"),
                &LevelInfo,
            )
            .await;

        // Upgrading the client

        let upgrade_client_payload = chain_a
            .upgrade_client_payload(&onchain_final_height)
            .await
            .map_err(ChainDriverA::raise_error)?;

        let upgrade_client_message = chain_b
            .upgrade_client_message(client_id_b, &upgrade_client_payload)
            .await
            .map_err(ChainDriverA::raise_error)?;

        chain_b
            .send_message_with_signer(chain_b.get_default_signer(), upgrade_client_message)
            .await
            .map_err(ChainDriverA::raise_error)?;

        chain_a
            .log(
                &format!(
                    "Upgraded Starknet client for upgrade height {}",
                    setup_result.upgrade_height
                ),
                &LevelInfo,
            )
            .await;

        let client_b_state = chain_b
            .query_client_state_with_latest_height(PhantomData, client_id_b)
            .await
            .map_err(ChainDriverA::raise_error)?;

        let resume_options = StarknetFullNodeResumeOptions {
            sequencer_private_key: setup_result.sequencer_private_key,
        };

        // Restart starknet chain with the new sequencer key
        chain_driver_a.halt_full_node().await?;
        let chain_driver_a = chain_driver_a.resume_full_node(&resume_options).await?;

        let chain_a = chain_driver_a.chain();
        let encoding = chain_a.encoding();

        // Check the updated sequencer `public_key`

        let endpoint_url = chain_a.feeder_gateway_url();
        let endpoint = FeederGatewayEndpoint::new(endpoint_url.as_str());
        let public_key = endpoint.get_public_key(None).unwrap();

        assert_eq!(
            public_key,
            get_public_key(&setup_result.sequencer_private_key)
        );

        // Post-upgrade, unschedule the upgrade on Starknet

        let ibc_core_contract_address = chain_a
            .query_contract_address(PhantomData)
            .await
            .map_err(ChainDriverA::raise_error)?;

        chain_a
            .send_message_with_signer(
                chain_a.get_default_signer(),
                StarknetMessage::new(
                    *ibc_core_contract_address,
                    selector!("unschedule_upgrade"),
                    vec![],
                ),
            )
            .await
            .map_err(ChainDriverA::raise_error)?;

        // Post-upgrade, Starknet client state must have upgraded

        let client_b_state = chain_b
            .query_client_state_with_latest_height(PhantomData, client_id_b)
            .await
            .map_err(ChainDriverA::raise_error)?;

        let client_b_state_height = ChainA::client_state_latest_height(&client_b_state);

        // Assert the client has been upgraded to the upgraded height
        assert_eq!(
            client_b_state_height,
            // Starknet upgrade height is the immediate next height
            setup_result.upgrade_height,
        );

        // Post-upgrade, update Starknet client must succeed

        let chain_a_height = chain_a
            .query_chain_height()
            .await
            .map_err(ChainDriverA::raise_error)?;

        assert!(setup_result.upgrade_height < chain_a_height);

        let client_b_update_payload = chain_a
            .build_update_client_payload(&client_b_state_height, &chain_a_height, client_b_state)
            .await
            .map_err(ChainDriverA::raise_error)?;

        let update_messages = chain_b
            .build_update_client_message(client_id_b, client_b_update_payload)
            .await
            .map_err(ChainDriverA::raise_error)?;

        chain_b
            .send_messages(update_messages)
            .await
            .map_err(ChainDriverA::raise_error)?;

        let client_b_state = chain_b
            .query_client_state_with_latest_height(PhantomData, client_id_b)
            .await
            .map_err(ChainDriverA::raise_error)?;

        let client_b_state_height = ChainA::client_state_latest_height(&client_b_state);

        assert_eq!(client_b_state_height, chain_a_height);

        chain_a
            .log(
                &format!("Updated Starknet client at post-upgrade height({chain_a_height})"),
                &LevelInfo,
            )
            .await;

        Ok(())
    }
}

pub struct SetupStarknetUpgradeClientTest;

#[cgp_provider(SetupUpgradeClientTestHandlerComponent)]
impl<ChainDriverA, ChainDriverB, ChainA, ChainB, Encoding>
    SetupUpgradeClientTestHandler<ChainDriverA, ChainDriverB> for SetupStarknetUpgradeClientTest
where
    ChainDriverA: HasChain<Chain = ChainA>
        + HasSetupUpgradeClientTestResultType<
            SetupUpgradeClientTestResult = StarknetProposalSetupClientUpgradeResult,
        > + CanRaiseAsyncError<ChainA::Error>
        + CanRaiseAsyncError<ChainB::Error>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<ClientError>,
    ChainDriverB: HasChain<Chain = ChainB>,
    ChainA: CanBuildClientUpgradePayload<ChainB>
        + CanQueryChainStatus<ChainStatus = StarknetChainStatus>
        + CanWaitChainReachHeight
        + HasDefaultSigner
        + CanSendSingleMessageWithSigner<Message = StarknetMessage>
        + CanLog<LevelInfo>
        + CanQueryChainHeight<Height = u64>
        + CanBuildUpdateClientPayload<ChainB>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address"), Address = StarknetAddress>
        + HasClientStateType<ChainB, ClientState = WasmStarknetClientState>
        + HasConsensusStateType<ChainB, ConsensusState = WasmStarknetConsensusState>
        + HasClientStateFields<ChainB, ChainId = ChainId>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + HasAsyncErrorType,
    ChainB: CanUpgradeClient<ChainA>
        + CanQueryClientStateWithLatestHeight<ChainA>
        + CanQueryConsensusStateWithLatestHeight<ChainA>
        + HasClientStateType<ChainA>
        + CanBuildUpdateClientMessage<ChainA>,
    Encoding: CanEncode<ViaCairo, Product![u64, CairoStarknetClientState, CairoStarknetConsensusState]>
        + HasAsyncErrorType
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn setup_upgrade_client_test(
        chain_driver_a: &ChainDriverA,
        chain_driver_b: &ChainDriverB,
        client_id_b: &ChainB::ClientId,
    ) -> Result<StarknetProposalSetupClientUpgradeResult, ChainDriverA::Error> {
        let chain_a = chain_driver_a.chain();

        let chain_b = chain_driver_b.chain();

        let latest_height = chain_a
            .query_chain_height()
            .await
            .map_err(ChainDriverA::raise_error)?;

        let chain_status = chain_a
            .query_chain_status()
            .await
            .map_err(ChainDriverA::raise_error)?;

        let wasm_client_state = chain_b
            .query_client_state_with_latest_height(PhantomData, client_id_b)
            .await
            .map_err(ChainDriverA::raise_error)?;

        let wasm_consensus_state = chain_b
            .query_consensus_state_with_latest_height(
                PhantomData,
                client_id_b,
                &wasm_client_state
                    .client_state
                    .latest_height
                    .revision_height(),
            )
            .await
            .map_err(ChainDriverA::raise_error)?;

        let mut upgrade_client_state: CairoStarknetClientState =
            wasm_client_state.client_state.into();
        let mut upgrade_consensus_state: CairoStarknetConsensusState =
            wasm_consensus_state.consensus_state.into();

        let final_height = chain_status.height + 30; // 1 sec block time; 30 secs in future
        let upgrade_height = final_height + 1;
        let upgrade_timestamp = Timestamp::from_nanoseconds(
            chain_status.time.unix_timestamp_nanos() as u64 + 30 * 1_000_000_000,
        );

        let ibc_core_contract_address = chain_a
            .query_contract_address(PhantomData::<symbol!("ibc_core_contract_address")>)
            .await
            .map_err(ChainDriverA::raise_error)?;

        // An arbitrary chosen value for testing purposes
        let sequencer_private_key = Felt::THREE;
        let sequencer_public_key = get_public_key(&sequencer_private_key);

        upgrade_client_state.latest_height = Height {
            revision_number: 0,
            revision_height: upgrade_height,
        };
        upgrade_client_state.final_height = 0;
        // upgrade_client_state.client_state.chain_id remains unchanged
        upgrade_client_state.sequencer_public_key = sequencer_public_key;
        upgrade_client_state.ibc_contract_address = ibc_core_contract_address;

        upgrade_consensus_state.root = Felt::ZERO;
        upgrade_consensus_state.time = upgrade_timestamp;

        let cairo_encoding = chain_a.encoding();

        let calldata = cairo_encoding
            .encode(&product![
                final_height,
                upgrade_client_state,
                upgrade_consensus_state
            ])
            .map_err(ChainDriverA::raise_error)?;

        let ibc_core_contract_address = chain_a
            .query_contract_address(PhantomData)
            .await
            .map_err(ChainDriverA::raise_error)?;

        chain_a
            .send_message_with_signer(
                chain_a.get_default_signer(),
                StarknetMessage::new(
                    *ibc_core_contract_address,
                    selector!("schedule_upgrade"),
                    calldata,
                ),
            )
            .await
            .map_err(ChainDriverA::raise_error)?;

        chain_a
            .log(
                &format!(
                    "Scheduled upgrade for Starknet client for upgrade height {upgrade_height}"
                ),
                &LevelInfo,
            )
            .await;

        Ok(StarknetProposalSetupClientUpgradeResult {
            upgrade_height,
            sequencer_private_key,
        })
    }
}
