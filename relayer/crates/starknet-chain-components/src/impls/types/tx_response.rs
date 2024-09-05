use cgp::core::Async;
use hermes_relayer_components::transaction::traits::types::tx_response::ProvideTxResponseType;

use crate::types::tx_response::TxResponse;

pub struct ProvideStarknetTxResponse;

impl<Chain: Async> ProvideTxResponseType<Chain> for ProvideStarknetTxResponse {
    type TxResponse = TxResponse;
}
