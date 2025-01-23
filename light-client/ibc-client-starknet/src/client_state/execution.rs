use cgp::core::component::UseContext;
use hermes_cosmos_encoding_components::impls::any::ConvertIbcAny;
use hermes_encoding_components::impls::convert::ConvertVia;
use hermes_encoding_components::traits::convert::Converter;
use ibc_client_cw::context::client_ctx::CwClientExecution;
use ibc_client_starknet_types::header::{
    SignedStarknetHeader, StarknetHeader, STARKNET_HEADER_TYPE_URL,
};
use ibc_client_starknet_types::StarknetClientState as ClientStateType;
use ibc_core::client::context::client_state::ClientStateExecution;
use ibc_core::client::context::prelude::{ClientStateCommon, ConsensusState};
use ibc_core::client::context::ClientExecutionContext;
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::host::types::path::{ClientConsensusStatePath, ClientStatePath};
use ibc_core::primitives::proto::Any;
use prost_types::Any as ProstAny;

use super::ClientState;
use crate::encoding::context::StarknetLightClientEncoding;
use crate::ConsensusState as StarknetConsensusState;

impl<'a, E> ClientStateExecution<E> for ClientState
where
    E: CwClientExecution<
        'a,
        ClientStateMut = ClientState,
        ConsensusStateRef = StarknetConsensusState,
    >,
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
        let signed_header: SignedStarknetHeader =
            <ConvertVia<ProstAny, ConvertIbcAny, UseContext>>::convert(
                &StarknetLightClientEncoding,
                &header,
            )?;

        let raw_header = signed_header.header;

        let header_digest = ctx.generate_sha256_digest(&raw_header);

        let deps = ctx
            .cosmwasm_execute_context()
            .ok_or_else(|| ClientError::ClientSpecific {
                description: "missing Deps from context".to_owned(),
            })?;

        match deps.api.secp256k1_verify(
            header_digest.as_slice(),
            &signed_header.signature,
            &self.0.pub_key,
        ) {
            Ok(validation) if validation => {}
            Ok(_) => {
                return Err(ClientError::ClientSpecific {
                    description: "Header signature not valid".to_owned(),
                })
            }
            Err(e) => {
                return Err(ClientError::ClientSpecific {
                    description: format!("Header signature verification failed:{e} "),
                })
            }
        }

        let any_header = Any {
            type_url: STARKNET_HEADER_TYPE_URL.to_owned(),
            value: raw_header,
        };

        let header: StarknetHeader = <ConvertVia<ProstAny, ConvertIbcAny, UseContext>>::convert(
            &StarknetLightClientEncoding,
            &any_header,
        )?;

        let latest_height = header.height;

        let new_consensus_state = header.consensus_state;

        let new_client_state = ClientStateType {
            latest_height: header.height,
            chain_id: self.0.chain_id.clone(),
            pub_key: self.0.pub_key.clone(),
        }
        .into();

        update_client_and_consensus_state(
            ctx,
            latest_height,
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
    let timestamp = consensus_state.timestamp()?;
    ctx.store_consensus_state(
        ClientConsensusStatePath::new(
            client_id.clone(),
            client_height.revision_number(),
            client_height.revision_height(),
        ),
        consensus_state,
    )?;
    ctx.store_client_state(ClientStatePath::new(client_id.clone()), client_state)?;
    ctx.store_update_meta(client_id.clone(), client_height, timestamp, Height::min(0))?;

    Ok(())
}
