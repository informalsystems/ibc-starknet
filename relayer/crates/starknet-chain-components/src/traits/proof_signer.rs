use cgp::prelude::*;

#[cgp_component {
  name: StarknetProofSignerTypeComponent,
  provider: ProvideStarknetProofSignerType,
  context: Chain,
}]
pub trait HasStarknetProofSignerType: Async {
    type ProofSigner: Async;
}

#[cgp_component {
  name: StarknetProofSignerGetterComponent,
  provider: StarknetProofSignerGetter,
  context: Chain,
}]
pub trait HasStarknetProofSigner: HasStarknetProofSignerType {
    fn proof_signer(&self) -> &Self::ProofSigner;
}

pub trait CanRaiseProofSignerErrors:
    HasStarknetProofSignerType + CanRaiseAsyncError<&'static str>
{
}

impl<Chain> CanRaiseProofSignerErrors for Chain where
    Chain: HasStarknetProofSignerType + CanRaiseAsyncError<&'static str>
{
}
