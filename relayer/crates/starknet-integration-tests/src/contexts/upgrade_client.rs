use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::impls::CanWaitChainReachHeight;
use hermes_core::chain_components::traits::{
    CanBuildClientUpgradePayload, CanBuildUpdateClientMessage, CanBuildUpdateClientPayload,
    CanQueryChainHeight, CanQueryChainStatus, CanQueryClientStateWithLatestHeight,
    CanQueryConsensusStateWithLatestHeight, CanUpgradeClient, HasClientStateFields,
    HasClientStateType, HasConsensusStateType,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelDebug;
use hermes_core::relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_core::relayer_components::multi::traits::client_id_at::HasClientIdAt;
use hermes_core::relayer_components::transaction::impls::CanSendSingleMessageWithSigner;
use hermes_core::relayer_components::transaction::traits::HasDefaultSigner;
use hermes_core::test_components::chain_driver::traits::{
    HasChain, HasSetupUpgradeClientTestResultType,
};
use hermes_core::test_components::driver::traits::HasChainDriverAt;
use hermes_core::test_components::test_case::traits::upgrade_client::{
    SetupUpgradeClientTestHandler, SetupUpgradeClientTestHandlerComponent, UpgradeClientHandler,
    UpgradeClientHandlerComponent,
};
use hermes_prelude::*;
use hermes_starknet_chain_components::impls::{StarknetAddress, StarknetMessage};
use hermes_starknet_chain_components::traits::{
    CanCallContract, CanQueryContractAddress, HasBlobType, HasSelectorType,
};
use hermes_starknet_chain_components::types::{
    CairoStarknetClientState, CairoStarknetConsensusState, Height, StarknetChainStatus,
    WasmStarknetClientState, WasmStarknetConsensusState,
};
use hermes_starknet_test_components::impls::StarknetProposalSetupClientUpgradeResult;
use ibc::core::client::types::error::ClientError;
use ibc::core::host::types::identifiers::ChainId;
use ibc::primitives::Timestamp;
use starknet::core::types::Felt;
use starknet::macros::selector;
use starknet_crypto::get_public_key;
pub struct StarknetHandleUpgradeClient;

#[cgp_provider(UpgradeClientHandlerComponent)]
impl<Driver, ChainDriverA, ChainDriverB, ChainA, ChainB, Encoding>
    UpgradeClientHandler<Driver, ChainDriverA, ChainA, ChainB> for StarknetHandleUpgradeClient
where
    Driver: HasChainDriverAt<Index<0>, ChainDriver = ChainDriverA>
        + HasChainDriverAt<Index<1>, ChainDriver = ChainDriverB>
        + HasClientIdAt<Index<1>, Index<0>>
        + HasChainTypeAt<Index<0>, Chain = ChainA>
        + HasChainTypeAt<Index<1>, Chain = ChainB>
        + CanLog<LevelDebug>
        + CanRaiseAsyncError<ChainA::Error>
        + CanRaiseAsyncError<ChainB::Error>
        + CanRaiseAsyncError<ClientError>
        + CanRaiseAsyncError<Encoding::Error>,
    ChainDriverA: HasChain<Chain = ChainA>
        + HasSetupUpgradeClientTestResultType<
            SetupUpgradeClientTestResult = StarknetProposalSetupClientUpgradeResult,
        >,
    ChainDriverB: HasChain<Chain = ChainB>,
    ChainA: CanBuildClientUpgradePayload<ChainB>
        + CanWaitChainReachHeight
        + CanQueryChainHeight<Height = u64>
        + CanBuildUpdateClientPayload<ChainB>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address"), Address = StarknetAddress>
        + CanCallContract
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + HasClientStateType<ChainB>
        + HasClientStateFields<ChainB, ChainId = ChainId>
        + HasAsyncErrorType,
    ChainB: CanUpgradeClient<ChainA>
        + HasDefaultSigner
        + CanSendSingleMessageWithSigner
        + CanQueryClientStateWithLatestHeight<ChainA>
        + CanBuildUpdateClientMessage<ChainA>,
    Encoding: CanDecode<ViaCairo, u64> + HasEncodedType<Encoded = Vec<Felt>> + HasAsyncErrorType,
{
    async fn handle_upgrade_client(
        driver: &Driver,
        setup_result: &StarknetProposalSetupClientUpgradeResult,
    ) -> Result<(), Driver::Error> {
        let chain_driver_a = driver.chain_driver_at(PhantomData::<Index<0>>);

        let chain_a = chain_driver_a.chain();

        let chain_driver_b = driver.chain_driver_at(PhantomData::<Index<1>>);

        let chain_b = chain_driver_b.chain();

        let client_id_b = driver.client_id_at(PhantomData::<(Index<1>, Index<0>)>);

        let encoding = chain_a.encoding();

        let ibc_core_contract_address = chain_a
            .query_contract_address(PhantomData)
            .await
            .map_err(Driver::raise_error)?;

        let output = chain_a
            .call_contract(
                &ibc_core_contract_address,
                &selector!("get_final_height"),
                &vec![],
                None,
            )
            .await
            .map_err(Driver::raise_error)?;

        let onchain_final_height: u64 = encoding.decode(&output).map_err(Driver::raise_error)?;

        chain_a
            .wait_chain_reach_height(&onchain_final_height)
            .await
            .map_err(Driver::raise_error)?;

        // update till final height
        // starknet_to_cosmos_relay
        //         .send_target_update_client_messages(DestinationTarget, &onchain_final_height)
        //         .await?;

        let upgrade_client_payload = chain_a
            .upgrade_client_payload(&onchain_final_height)
            .await
            .map_err(Driver::raise_error)?;

        let upgrade_client_message = chain_b
            .upgrade_client_message(client_id_b, &upgrade_client_payload)
            .await
            .map_err(Driver::raise_error)?;

        chain_b
            .send_message_with_signer(chain_b.get_default_signer(), upgrade_client_message)
            .await
            .map_err(Driver::raise_error)?;

        let client_b_state = chain_b
            .query_client_state_with_latest_height(PhantomData, client_id_b)
            .await
            .map_err(Driver::raise_error)?;

        // FIXME(rano): do some asserts
        // Assert the client has been upgraded to the new chain ID
        // assert_eq!(
        //     ChainA::client_state_latest_height(&client_b_state),
        //     setup_result.new_chain_id.clone(),
        // );

        Ok(())
    }
}

pub struct SetupStarknetUpgradeClientTest;

#[cgp_provider(SetupUpgradeClientTestHandlerComponent)]
impl<Driver, ChainDriverA, ChainDriverB, ChainA, ChainB, Encoding>
    SetupUpgradeClientTestHandler<Driver, ChainDriverA, ChainA, ChainB>
    for SetupStarknetUpgradeClientTest
where
    Driver: HasChainDriverAt<Index<0>, ChainDriver = ChainDriverA>
        + HasChainDriverAt<Index<1>, ChainDriver = ChainDriverB>
        + HasClientIdAt<Index<1>, Index<0>>
        + HasChainTypeAt<Index<0>, Chain = ChainA>
        + HasChainTypeAt<Index<1>, Chain = ChainB>
        + CanLog<LevelDebug>
        + CanRaiseAsyncError<ChainA::Error>
        + CanRaiseAsyncError<ChainB::Error>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<ClientError>,
    ChainDriverA: HasChain<Chain = ChainA>
        + HasSetupUpgradeClientTestResultType<
            SetupUpgradeClientTestResult = StarknetProposalSetupClientUpgradeResult,
        >,
    ChainDriverB: HasChain<Chain = ChainB>,
    ChainA: CanBuildClientUpgradePayload<ChainB>
        + CanQueryChainStatus<ChainStatus = StarknetChainStatus>
        + CanWaitChainReachHeight
        + HasDefaultSigner
        + CanSendSingleMessageWithSigner<Message = StarknetMessage>
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
        driver: &Driver,
    ) -> Result<StarknetProposalSetupClientUpgradeResult, Driver::Error> {
        let chain_driver_a = driver.chain_driver_at(PhantomData::<Index<0>>);

        let chain_a = chain_driver_a.chain();

        let chain_driver_b = driver.chain_driver_at(PhantomData::<Index<1>>);

        let chain_b = chain_driver_b.chain();

        let client_id_b = driver.client_id_at(PhantomData::<(Index<1>, Index<0>)>);

        let latest_height = chain_a
            .query_chain_height()
            .await
            .map_err(Driver::raise_error)?;

        let chain_status = chain_a
            .query_chain_status()
            .await
            .map_err(Driver::raise_error)?;

        let wasm_client_state = chain_b
            .query_client_state_with_latest_height(PhantomData, client_id_b)
            .await
            .map_err(Driver::raise_error)?;

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
            .map_err(Driver::raise_error)?;

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
            .map_err(Driver::raise_error)?;

        // FIXME(rano): use random bits to generate this
        let sequencer_private_key = Felt::TWO;
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
            .map_err(Driver::raise_error)?;

        let ibc_core_contract_address = chain_a
            .query_contract_address(PhantomData)
            .await
            .map_err(Driver::raise_error)?;

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
            .map_err(Driver::raise_error)?;

        Ok(StarknetProposalSetupClientUpgradeResult {
            sequencer_private_key,
        })
    }
}
