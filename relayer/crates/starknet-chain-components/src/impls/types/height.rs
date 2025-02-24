use cgp::prelude::*;
use hermes_chain_components::traits::types::height::{
    HeightAdjuster, HeightAdjusterComponent, HeightFieldComponent, HeightIncrementer,
    HeightIncrementerComponent, HeightTypeComponent,
};
use hermes_relayer_components::chain::traits::types::height::{
    HasHeightType, HeightFieldGetter, ProvideHeightType,
};

pub struct ProvideStarknetHeight;

#[cgp_provider(HeightTypeComponent)]
impl<Chain: Async> ProvideHeightType<Chain> for ProvideStarknetHeight {
    type Height = u64;
}

#[cgp_provider(HeightFieldComponent)]
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

#[cgp_provider(HeightIncrementerComponent)]
impl<Chain> HeightIncrementer<Chain> for ProvideStarknetHeight
where
    Chain: HasHeightType<Height = u64> + CanRaiseAsyncError<&'static str>,
{
    fn increment_height(height: &u64) -> Result<u64, Chain::Error> {
        height
            .checked_add(1)
            .ok_or_else(|| Chain::raise_error("u64 overflow"))
    }
}

#[cgp_provider(HeightAdjusterComponent)]
impl<Chain> HeightAdjuster<Chain> for ProvideStarknetHeight
where
    Chain: HasHeightType<Height = u64> + CanRaiseAsyncError<&'static str>,
{
    fn add_height(height: &u64, addition: u64) -> Result<u64, Chain::Error> {
        height
            .checked_add(addition)
            .ok_or_else(|| Chain::raise_error("u64 overflow"))
    }

    fn sub_height(height: &u64, subtraction: u64) -> Result<u64, Chain::Error> {
        height
            .checked_sub(subtraction)
            .ok_or_else(|| Chain::raise_error("u64 underflow"))
    }
}
