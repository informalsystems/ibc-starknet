use hermes_prelude::*;
use hermes_core::relayer_components::transaction::traits::HasSignerType;

#[cgp_getter {
    provider: StarknetSignerGetter,
    context: Chain,
}]
pub trait HasStarknetSigner: HasSignerType {
    fn signer(&self) -> &Self::Signer;
}
