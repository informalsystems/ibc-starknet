use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::message::ProvideMessageType;
use starknet::accounts::Call;

pub struct ProvideCallMessage;

impl<Chain: Async> ProvideMessageType<Chain> for ProvideCallMessage {
    type Message = Call;
}
