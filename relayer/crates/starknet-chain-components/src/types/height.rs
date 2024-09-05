use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::height::{
    HasHeightType, HeightFieldGetter, ProvideHeightType,
};

pub struct ProvideStarknetHeight;

impl<Chain: Async> ProvideHeightType<Chain> for ProvideStarknetHeight {
    type Height = u64;
}

impl<Chain> HeightFieldGetter<Chain> for ProvideStarknetHeight
where
    Chain: HasHeightType<Height = u64>,
{
    fn revision_number(_height: &u64) -> u64 {
        0
    }

    fn revision_height(height: &u64) -> u64 {
        *height
    }
}
