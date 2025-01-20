use std::sync::Arc;

use cgp::prelude::*;
use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;

use crate::traits::queries::address::CanQueryContractAddress;

pub trait CanCreateStarknetSubscription: HasHeightType + HasEventType + HasAsyncErrorType {
    fn create_event_subscription(
        &self,
    ) -> Result<Arc<dyn Subscription<Item = (Self::Height, Arc<Self::Event>)>>, Self::Error>;
}

impl<Chain> CanCreateStarknetSubscription for Chain
where
    Chain: HasHeightType
        + HasEventType
        + HasAsyncErrorType
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>,
{
    fn create_event_subscription(
        &self,
    ) -> Result<Arc<dyn Subscription<Item = (Self::Height, Arc<Self::Event>)>>, Self::Error> {
        todo!()
    }
}
