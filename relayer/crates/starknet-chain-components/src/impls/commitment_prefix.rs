use std::sync::LazyLock;

use cgp::prelude::*;
use hermes_core::chain_components::traits::{
    HasCommitmentPrefixType, IbcCommitmentPrefixGetter, IbcCommitmentPrefixGetterComponent,
};

pub struct GetStarknetCommitmentPrefix;

#[cgp_provider(IbcCommitmentPrefixGetterComponent)]
impl<Chain> IbcCommitmentPrefixGetter<Chain> for GetStarknetCommitmentPrefix
where
    Chain: HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>,
{
    fn ibc_commitment_prefix(_chain: &Chain) -> &Vec<u8> {
        // FIXME: Use the Cairo IBC Core contract address as commitment prefix
        static IBC_COMMITMENT_PREFIX: LazyLock<Vec<u8>> = LazyLock::new(|| "ibc".into());

        &IBC_COMMITMENT_PREFIX
    }
}
