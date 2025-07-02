use core::fmt::Debug;

use hermes_core::test_components::chain::traits::{
    HasProposalIdType, HasProposalStatusType, ProposalStatusQuerier, ProposalStatusQuerierComponent,
};
use hermes_cosmos_chain_components::traits::HasGrpcAddress;
use hermes_prelude::*;
use hermes_test_components::chain::types::ProposalStatus;
use http::uri::InvalidUri;
use http::Uri;
use ibc_proto::cosmos::gov::v1::query_client::QueryClient;
use ibc_proto::cosmos::gov::v1::{Proposal, QueryProposalRequest};
use tonic::transport::Error as TransportError;
use tonic::Status;

pub struct QueryProposalStatusAlwaysDeposit;

#[cgp_provider(ProposalStatusQuerierComponent)]
impl<Chain> ProposalStatusQuerier<Chain> for QueryProposalStatusWithGrpc
where
    Chain: HasProposalIdType<ProposalId = u64>
        + HasProposalStatusType<ProposalStatus = ProposalStatus>
        + HasGrpcAddress
        + CanRaiseAsyncError<InvalidUri>
        + CanRaiseAsyncError<Status>
        + CanRaiseAsyncError<TransportError>
        + CanRaiseAsyncError<String>
        + for<'a> CanRaiseAsyncError<ProposalFailed<'a, Chain>>,
{
    async fn query_proposal_status(
        _chain: &Chain,
        _proposal_id: &u64,
    ) -> Result<ProposalStatus, Chain::Error> {
        Ok(ProposalStatus::DepositPeriod)
    }
}
