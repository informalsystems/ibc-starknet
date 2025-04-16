use cometbft::utils::ONE_THIRD;
use ics23::tendermint_spec;
use starknet_ibc_clients::cometbft::{
    CometClientState, CometConsensusState, CometHeader, SignedHeader,
};
use starknet_ibc_core::client::{
    CreateResponse, Duration, Height, MsgCreateClient, MsgRecoverClient, MsgUpdateClient, Status,
    Timestamp,
};
use starknet_ibc_core::host::ClientId;
use starknet_ibc_testkit::dummies::{
    CLIENT_TYPE, DURATION, HEIGHT, NEXT_VALIDATOR_HASH, STATE_ROOT, TIMESTAMP,
};
use starknet_ibc_testkit::handles::{CoreContract, CoreHandle};

#[derive(Clone, Debug, Drop, Serde)]
pub struct CometClientConfig {
    pub client_type: felt252,
    pub latest_height: Height,
    pub latest_timestamp: Timestamp,
    pub trusting_period: Duration,
    pub unbonding_period: Duration,
    pub max_clock_drift: Duration,
}

#[generate_trait]
pub impl CometClientConfigImpl of CometClientConfigTrait {
    fn default() -> CometClientConfig {
        CometClientConfig {
            client_type: CLIENT_TYPE(),
            latest_height: HEIGHT(10),
            latest_timestamp: TIMESTAMP(10),
            trusting_period: DURATION(100),
            unbonding_period: DURATION(200),
            max_clock_drift: DURATION(1),
        }
    }

    fn dummy_msg_create_client(self: @CometClientConfig) -> MsgCreateClient {
        let mut serialized_client_state: Array<felt252> = ArrayTrait::new();

        let client_state = CometClientState {
            latest_height: self.latest_height.clone(),
            trusting_period: *self.trusting_period,
            unbonding_period: *self.unbonding_period,
            max_clock_drift: *self.max_clock_drift,
            trust_level: ONE_THIRD,
            status: Status::Active,
            chain_id: "dummy_chain",
            proof_spec: array![tendermint_spec()],
        };

        Serde::serialize(@client_state, ref serialized_client_state);

        let mut serialized_consensus_state: Array<felt252> = ArrayTrait::new();

        let consensus_state = CometConsensusState {
            timestamp: self.latest_timestamp.clone().into(),
            root: STATE_ROOT(),
            next_validators_hash: NEXT_VALIDATOR_HASH(),
        };

        Serde::serialize(@consensus_state, ref serialized_consensus_state);

        MsgCreateClient {
            client_type: *self.client_type,
            client_state: serialized_client_state,
            consensus_state: serialized_consensus_state,
        }
    }

    fn dummy_msg_update_client(
        self: @CometClientConfig,
        client_id: ClientId,
        trusted_height: Height,
        latest_height: Height,
        latest_timestamp: Timestamp,
    ) -> MsgUpdateClient {
        let mut serialized_header: Array<felt252> = ArrayTrait::new();

        let signed_header = SignedHeader {
            height: latest_height,
            timestamp: latest_timestamp,
            root: STATE_ROOT(),
            next_validators_hash: NEXT_VALIDATOR_HASH(),
            protobuf_bytes: ArrayTrait::new(),
        };

        let header = CometHeader { trusted_height, signed_header };

        Serde::serialize(@header, ref serialized_header);

        MsgUpdateClient { client_id, client_message: serialized_header }
    }

    fn dummy_msg_recover_client(
        self: @CometClientConfig, subject_client_id: ClientId, substitute_client_id: ClientId,
    ) -> MsgRecoverClient {
        MsgRecoverClient { subject_client_id, substitute_client_id }
    }

    fn create_client(self: @CometClientConfig, core: @CoreContract) -> CreateResponse {
        core.create_client(self.dummy_msg_create_client())
    }
}
