use cgp::prelude::*;
use hermes_cosmos_chain_components::components::transaction::TxResponseTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_response::ProvideTxResponseType;

use crate::types::tx_response::TxResponse;

pub struct ProvideStarknetTxResponse;

#[cgp_provider(TxResponseTypeComponent)]
impl<Chain: Async> ProvideTxResponseType<Chain> for ProvideStarknetTxResponse {
    type TxResponse = TxResponse;
}
