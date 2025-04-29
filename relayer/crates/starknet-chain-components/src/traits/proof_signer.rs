use hermes_prelude::*;

#[cgp_type {
    provider: StarknetProofSignerTypeProvider,
    context: Chain,
}]
pub trait HasStarknetProofSignerType: Async {
    type ProofSigner: Async;
}

#[cgp_getter {
    provider: StarknetProofSignerGetter,
    context: Chain,
}]
pub trait HasStarknetProofSigner: HasStarknetProofSignerType {
    fn proof_signer(&self) -> &Self::ProofSigner;
}
