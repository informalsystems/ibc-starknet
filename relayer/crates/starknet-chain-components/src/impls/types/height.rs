use cgp::core::Async;
use cgp::prelude::CanRaiseError;
use hermes_chain_components::traits::types::height::HeightIncrementer;
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

impl<Chain> HeightIncrementer<Chain> for ProvideStarknetHeight
where
    Chain: HasHeightType<Height = u64> + CanRaiseError<&'static str>,
{
    fn increment_height(height: &u64) -> Result<u64, Chain::Error> {
        height
            .checked_add(1)
            .ok_or_else(|| Chain::raise_error("u64 overflow"))
    }
}
