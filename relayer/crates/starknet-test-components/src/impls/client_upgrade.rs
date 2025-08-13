use starknet::core::types::Felt;

#[derive(Clone, Debug)]
pub struct StarknetProposalSetupClientUpgradeResult {
    pub sequencer_private_key: Felt,
}
