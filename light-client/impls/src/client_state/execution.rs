use ibc_client_starknet_types::ClientState as ClientStateType;
use ibc_core::client::context::client_state::ClientStateExecution;
use ibc_core::client::context::prelude::ClientStateCommon;
use ibc_core::client::context::ClientExecutionContext;
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::host::types::path::{ClientConsensusStatePath, ClientStatePath};
use ibc_core::primitives::proto::Any;
use ibc_core::primitives::Timestamp;

use super::ClientState;
use crate::ConsensusState;

impl<E> ClientStateExecution<E> for ClientState
where
    E: ClientExecutionContext<ClientStateMut = ClientState, ConsensusStateRef = ConsensusState>,
{
    fn initialise(
        &self,
        ctx: &mut E,
        client_id: &ClientId,
        consensus_state: Any,
    ) -> Result<(), ClientError> {
        let latest_height = Height::min(0);

        update_client_and_consensus_state(
            ctx,
            latest_height,
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
        let latest_height = ctx.client_state(client_id)?.latest_height().increment();

        let new_client_state = ClientStateType { latest_height }.into();

        update_client_and_consensus_state(
            ctx,
            latest_height,
            client_id,
            new_client_state,
            ConsensusState::default(),
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
        let latest_height = ctx.client_state(client_id)?.latest_height().increment();

        update_client_and_consensus_state(
            ctx,
            latest_height,
            client_id,
            upgraded_client_state.try_into()?,
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
        let latest_height = ctx
            .client_state(subject_client_id)?
            .latest_height()
            .increment();

        update_client_and_consensus_state(
            ctx,
            latest_height,
            subject_client_id,
            substitute_client_state.try_into()?,
            substitute_consensus_state.try_into()?,
        )?;

        Ok(())
    }
}

fn update_client_and_consensus_state<E: ClientExecutionContext>(
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
        Timestamp::none(),
        Height::min(0),
    )?;

    Ok(())
}
