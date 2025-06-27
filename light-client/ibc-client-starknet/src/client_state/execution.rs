use alloc::vec;
use alloc::vec::Vec;

use cgp::core::component::UseContext;
use hermes_cosmos_encoding_components::impls::ConvertIbcAny;
use hermes_encoding_components::impls::ConvertVia;
use hermes_encoding_components::traits::{CanDecode, Converter};
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use ibc_client_starknet_types::header::{SignedStarknetHeader, StarknetHeader};
use ibc_client_starknet_types::StarknetClientState as ClientStateType;
use ibc_core::client::context::client_state::ClientStateExecution;
use ibc_core::client::context::prelude::ClientStateCommon;
use ibc_core::client::context::{ClientExecutionContext, ExtClientExecutionContext};
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::host::types::path::{ClientConsensusStatePath, ClientStatePath};
use ibc_core::primitives::proto::Any;
use prost_types::Any as ProstAny;

use super::ClientState;
use crate::encoding::context::StarknetLightClientEncoding;
use crate::ConsensusState as StarknetConsensusState;

impl<E> ClientStateExecution<E> for ClientState
where
    E: ClientExecutionContext<ClientStateMut = Self, ConsensusStateRef = StarknetConsensusState>
        + ExtClientExecutionContext,
{
    fn initialise(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        consensus_state: Any,
    ) -> Result<(), ClientError> {
        update_client_and_consensus_state(
            ctx,
            self.latest_height(),
            client_id,
            self.clone(),
            consensus_state.try_into()?,
        )?;

        Ok(())
    }

    fn update_state(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        header: Any,
    ) -> Result<Vec<Height>, ClientError> {
        let signed_header: SignedStarknetHeader =
            <ConvertVia<ProstAny, ConvertIbcAny, UseContext>>::convert(
                &StarknetLightClientEncoding,
                &header,
            )?;

        let raw_header = signed_header.header;

        let header: StarknetHeader = <StarknetLightClientEncoding as CanDecode<
            ViaProtobuf,
            StarknetHeader,
        >>::decode(&StarknetLightClientEncoding, &raw_header)?;

        let current_height = header.height;

        let latest_height = core::cmp::max(self.latest_height(), current_height);

        let new_consensus_state = header.consensus_state;

        let new_client_state = ClientStateType {
            latest_height,
            chain_id: self.0.chain_id.clone(),
            pub_key: self.0.pub_key.clone(),
            ibc_contract_address: self.0.ibc_contract_address.clone(),
        }
        .into();

        update_client_and_consensus_state(
            ctx,
            current_height,
            client_id,
            new_client_state,
            new_consensus_state.into(),
        )?;

        Ok(vec![latest_height])
    }

    fn update_state_on_misbehaviour(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        client_message: Any,
    ) -> Result<(), ClientError> {
        Ok(())
    }

    fn update_state_on_upgrade(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        upgraded_client_state: Any,
        upgraded_consensus_state: Any,
    ) -> Result<Height, ClientError> {
        let client_state = E::ClientStateRef::try_from(upgraded_client_state)?;

        let latest_height = client_state.latest_height();

        update_client_and_consensus_state(
            ctx,
            latest_height,
            client_id,
            client_state,
            upgraded_consensus_state.try_into()?,
        )?;

        Ok(latest_height)
    }

    fn update_on_recovery(
        &self,
        ctx: &mut E,
        subject_client_id: &ClientId,
        substitute_client_state: Any,
        substitute_consensus_state: Any,
    ) -> Result<(), ClientError> {
        let client_state = E::ClientStateRef::try_from(substitute_client_state)?;

        update_client_and_consensus_state(
            ctx,
            client_state.latest_height(),
            subject_client_id,
            client_state,
            substitute_consensus_state.try_into()?,
        )?;

        Ok(())
    }
}

fn update_client_and_consensus_state<E: ExtClientExecutionContext>(
    ctx: &mut E,
    client_height: Height,
    client_id: &ClientId,
    client_state: E::ClientStateRef,
    consensus_state: E::ConsensusStateRef,
) -> Result<(), ClientError> {
    ctx.store_consensus_state(
        ClientConsensusStatePath::new(
            client_id.clone(),
            client_height.revision_number(),
            client_height.revision_height(),
        ),
        consensus_state,
    )?;
    ctx.store_client_state(ClientStatePath::new(client_id.clone()), client_state)?;
    ctx.store_update_meta(
        client_id.clone(),
        client_height,
        ctx.host_timestamp()?,
        ctx.host_height()?,
    )?;

    Ok(())
}
