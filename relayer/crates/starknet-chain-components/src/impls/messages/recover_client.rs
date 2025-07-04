use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    ClientRecovery, ClientRecoveryComponent, HasAddressType, HasClientIdType, HasMessageType,
    HasRecoverClientPayloadType,
};
use hermes_core::encoding_components::traits::{CanEncode, HasEncodedType, HasEncoding};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::ClientId;
use starknet::core::types::Felt;
use starknet::macros::selector;

use crate::impls::{StarknetAddress, StarknetMessage, StarknetRecoverClientPayload};
use crate::traits::CanQueryContractAddress;

pub struct RecoverStarknetClient;

#[cgp_provider(ClientRecoveryComponent)]
impl<Chain, Counterparty, Encoding> ClientRecovery<Chain, Counterparty> for RecoverStarknetClient
where
    Chain: HasClientIdType<Counterparty, ClientId = ClientId>
        + HasMessageType<Message = StarknetMessage>
        + HasAddressType<Address = StarknetAddress>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasRecoverClientPayloadType<RecoverClientPayload = StarknetRecoverClientPayload>,
    Encoding:
        HasEncodedType<Encoded = Vec<Felt>> + CanEncode<ViaCairo, Product![ClientId, ClientId]>,
{
    async fn recover_client_message(
        chain: &Chain,
        subject_client: &Chain::ClientId,
        substitute_client: &Chain::ClientId,
        _recover_client_payload: &StarknetRecoverClientPayload,
    ) -> Chain::Message {
        // TODO: Correctly implement building recover client message for Starknet
        let encoding = chain.encoding();
        let contract_address = chain
            .query_contract_address(PhantomData)
            .await
            .expect("Failed to query contract address");

        let calldata = encoding
            .encode(&product![subject_client.clone(), substitute_client.clone()])
            .expect("Failed to encode subject and substitute client IDs to Vec<Felt>");

        StarknetMessage::new(*contract_address, selector!("recover_client"), calldata)
    }
}
