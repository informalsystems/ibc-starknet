use starknet::core::types::Felt;

#[derive(Clone, Debug)]
pub struct StarknetProposalSetupClientUpgradeResult {
    pub upgrade_height: u64,
    pub sequencer_private_key: Felt,
}
