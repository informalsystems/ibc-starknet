use hermes_core::chain_components::traits::{
    ConsensusStateHeightsQuerier, ConsensusStateHeightsQuerierComponent, HasHeightType,
    HasIbcChainTypes,
};
use hermes_cosmos_core::chain_components::traits::HasGrpcAddress;
use hermes_prelude::*;
use http::uri::InvalidUri;
use http::Uri;
use ibc::core::host::types::identifiers::ClientId;
use ibc_proto::ibc::core::client::v1::query_client::QueryClient;
use ibc_proto::ibc::core::client::v1::QueryConsensusStateHeightsRequest;
use tonic::transport::Error as TransportError;
use tonic::Status;

pub struct QueryStarknetConsensusStateHeightsFromGrpc;

#[cgp_provider(ConsensusStateHeightsQuerierComponent)]
impl<Chain, Counterparty> ConsensusStateHeightsQuerier<Chain, Counterparty>
    for QueryStarknetConsensusStateHeightsFromGrpc
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + HasGrpcAddress
        + CanRaiseAsyncError<InvalidUri>
        + CanRaiseAsyncError<TransportError>
        + CanRaiseAsyncError<Status>,
    Counterparty: HasHeightType<Height = u64>,
{
    async fn query_consensus_state_heights(
        chain: &Chain,
        client_id: &ClientId,
    ) -> Result<Vec<u64>, Chain::Error> {
        let request = QueryConsensusStateHeightsRequest {
            client_id: client_id.to_string(),
            pagination: None,
        };

        let response = QueryClient::connect(
            Uri::try_from(&chain.grpc_address().to_string()).map_err(Chain::raise_error)?,
        )
        .await
        .map_err(Chain::raise_error)?
        .max_decoding_message_size(1 << 25)
        .consensus_state_heights(tonic::Request::new(request))
        .await
        .map_err(Chain::raise_error)?
        .into_inner();

        let mut heights: Vec<u64> = response
            .consensus_state_heights
            .into_iter()
            .map(|height| height.revision_height)
            .collect();

        heights.sort_unstable();

        Ok(heights)
    }
}
