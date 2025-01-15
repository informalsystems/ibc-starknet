use starknet_ibc_clients::cometbft::{
    CometClientState, CometConsensusState, CometHeader, SignedHeader
};
use starknet_ibc_core::client::CreateResponse;
use starknet_ibc_core::client::{MsgCreateClient, MsgUpdateClient, Height, Status};
use starknet_ibc_core::host::ClientId;
use starknet_ibc_testkit::dummies::{HEIGHT, CLIENT_TYPE, STATE_ROOT};
use starknet_ibc_testkit::handles::{CoreContract, CoreHandle};

#[derive(Clone, Debug, Drop, Serde)]
pub struct CometClientConfig {
    pub client_type: felt252,
    pub latest_height: Height,
    pub latest_timestamp: u64,
    pub trusting_period: u64,
    pub unbonding_period: u64,
}

#[generate_trait]
pub impl CometClientConfigImpl of CometClientConfigTrait {
    fn default() -> CometClientConfig {
        CometClientConfig {
            client_type: CLIENT_TYPE(),
            latest_height: HEIGHT(10),
            latest_timestamp: 10,
            trusting_period: 100,
            unbonding_period: 200,
        }
    }

    fn dummy_msg_create_client(self: @CometClientConfig) -> MsgCreateClient {
        let mut serialized_client_state: Array<felt252> = ArrayTrait::new();

        let client_state = CometClientState {
            latest_height: self.latest_height.clone(),
            trusting_period: *self.trusting_period,
            unbonding_period: *self.unbonding_period,
            status: Status::Active,
            chain_id: "dummy_chain",
        };

        Serde::serialize(@client_state, ref serialized_client_state);

        let mut serialized_consensus_state: Array<felt252> = ArrayTrait::new();

        let consensus_state = CometConsensusState {
            timestamp: self.latest_timestamp.clone().into(), root: STATE_ROOT()
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
        latest_timestamp: u64,
    ) -> MsgUpdateClient {
        let mut serialized_header: Array<felt252> = ArrayTrait::new();

        let signed_header = SignedHeader {
            height: latest_height, time: latest_timestamp, root: STATE_ROOT()
        };

        let header = CometHeader { trusted_height, signed_header };

        Serde::serialize(@header, ref serialized_header);

        MsgUpdateClient { client_id, client_message: serialized_header }
    }

    fn create_client(self: @CometClientConfig, core: @CoreContract) -> CreateResponse {
        core.create_client(self.dummy_msg_create_client())
    }
}
