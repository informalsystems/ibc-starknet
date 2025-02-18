use core::marker::PhantomData;

use cgp::prelude::*;

use crate::traits::proof_signer::{
    HasStarknetProofSignerType, ProvideStarknetProofSignerType, StarknetProofSignerGetter,
    StarknetProofSignerGetterComponent, StarknetProofSignerTypeComponent,
};

pub struct GetStarknetProofSignerField<Tag>(pub PhantomData<Tag>);

#[cgp_provider(StarknetProofSignerTypeComponent)]
impl<Chain, Tag> ProvideStarknetProofSignerType<Chain> for GetStarknetProofSignerField<Tag>
where
    Chain: Async + HasField<Tag>,
    Tag: Async,
    Chain::Value: Async,
{
    type ProofSigner = Chain::Value;
}

#[cgp_provider(StarknetProofSignerGetterComponent)]
impl<Chain, Tag> StarknetProofSignerGetter<Chain> for GetStarknetProofSignerField<Tag>
where
    Chain: Async + HasStarknetProofSignerType + HasField<Tag, Value = Chain::ProofSigner>,
    Tag: Async,
{
    fn proof_signer(chain: &Chain) -> &Chain::ProofSigner {
        chain.get_field(PhantomData)
    }
}
