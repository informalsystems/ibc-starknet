use hermes_core::chain_components::traits::HasClientIdType;
use hermes_core::test_components::test_case::traits::recover_client::{
    RecoverClientHandler, RecoverClientHandlerComponent,
};
use hermes_prelude::*;
use hermes_starknet_chain_components::types::ClientId;

pub struct RecoverStarknetClientHandler;

#[cgp_provider(RecoverClientHandlerComponent)]
impl<Driver, ChainDriverA, ChainA, ChainB>
    RecoverClientHandler<Driver, ChainDriverA, ChainA, ChainB> for RecoverStarknetClientHandler
where
    Driver: HasAsyncErrorType,
    ChainA: HasClientIdType<ChainB, ClientId = ClientId>,
{
    async fn handle_recover_client(
        driver: &Driver,
        subject_client_id: &ClientId,
        substitute_client_id: &ClientId,
    ) -> Result<(), Driver::Error> {
        // TODO: Add required logic to handle client recovery for Starknet client

        Ok(())
    }
}
