use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::HasHeightType;


pub trait CanCreateStarknetSubscription: HasHeightType + HasEventType {

}