use core::marker::PhantomData;
use core::time::Duration;

use hermes_core::relayer_components::transaction::traits::{
    ClientRefreshRateGetter, ClientRefreshRateGetterComponent,
};
use hermes_prelude::*;

#[cgp_new_provider(ClientRefreshRateGetterComponent)]
impl<Chain> ClientRefreshRateGetter<Chain> for GetStarknetClientRefreshRate
where
    Chain: HasField<symbol!("client_refresh_rate"), Value = Option<Duration>>,
{
    fn client_refresh_rate(chain: &Chain) -> &Option<Duration> {
        chain.get_field(PhantomData)
    }
}
