use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::timestamp::ProvideTimestampType;

pub struct ProvideStarknetTimestampType;

impl<Chain: Async> ProvideTimestampType<Chain> for ProvideStarknetTimestampType {
    // Dummy implementation for now
    type Timestamp = ();

    fn timestamp_duration_since(
        _earlier: &Self::Timestamp,
        _later: &Self::Timestamp,
    ) -> Option<std::time::Duration> {
        None
    }
}
