use cometbft::ibc::{Height, MerkleRoot};
use ics23::ProofSpec;
use protobuf::types::wkt::{Timestamp, Duration};
use cometbft::utils::Fraction;
use cometbft::types::{SignedHeader, ValidatorSet};

use protobuf::types::message::{
    ProtoMessage, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl, ProtoName,
};
use protobuf::primitives::array::{ByteArrayAsProtoMessage};
use protobuf::primitives::numeric::{I32AsProtoMessage, BoolAsProtoMessage};
use protobuf::types::tag::WireType;

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct ClientState {
    chain_id: ByteArray,
    trust_level: Fraction,
    trusting_period: Duration,
    unbonding_period: Duration,
    max_clock_drift: Duration,
    frozen_height: Height,
    latest_height: Height,
    proof_specs: Array<ProofSpec>,
    upgrade_path: Array<ByteArray>,
    allow_update_after_expiry: bool,
    allow_update_after_misbehaviour: bool,
}

impl ClientStateAsProtoMessage of ProtoMessage<ClientState> {
    fn encode_raw(self: @ClientState, ref context: EncodeContext) {
        context.encode_field(1, self.chain_id);
        context.encode_field(2, self.trust_level);
        context.encode_field(3, self.trusting_period);
        context.encode_field(4, self.unbonding_period);
        context.encode_field(5, self.max_clock_drift);
        context.encode_field(6, self.frozen_height);
        context.encode_field(7, self.latest_height);
        context.encode_repeated_field(8, self.proof_specs);
        context.encode_repeated_field(9, self.upgrade_path);
        context.encode_field(10, self.allow_update_after_expiry);
        context.encode_field(11, self.allow_update_after_misbehaviour);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<ClientState> {
        let mut client_state = Default::<ClientState>::default();
        if !context.decode_field(1, ref client_state.chain_id) {
            return Option::None;
        }
        if !context.decode_field(2, ref client_state.trust_level) {
            return Option::None;
        }
        if !context.decode_field(3, ref client_state.trusting_period) {
            return Option::None;
        }
        if !context.decode_field(4, ref client_state.unbonding_period) {
            return Option::None;
        }
        if !context.decode_field(5, ref client_state.max_clock_drift) {
            return Option::None;
        }
        if !context.decode_field(6, ref client_state.frozen_height) {
            return Option::None;
        }
        if !context.decode_field(7, ref client_state.latest_height) {
            return Option::None;
        }
        if !context.decode_repeated_field(8, ref client_state.proof_specs) {
            return Option::None;
        }
        if !context.decode_repeated_field(9, ref client_state.upgrade_path) {
            return Option::None;
        }
        if !context.decode_field(10, ref client_state.allow_update_after_expiry) {
            return Option::None;
        }
        if !context.decode_field(11, ref client_state.allow_update_after_misbehaviour) {
            return Option::None;
        }
        Option::Some(client_state)
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}


impl ClientStateAsProtoName of ProtoName<ClientState> {
    fn type_url() -> ByteArray {
        "ClientState"
    }
}


#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct ConsensusState {
    timestamp: Timestamp,
    root: MerkleRoot,
    next_validators_hash: ByteArray,
}

impl ConsensusStateAsProtoMessage of ProtoMessage<ConsensusState> {
    fn encode_raw(self: @ConsensusState, ref context: EncodeContext) {
        context.encode_field(1, self.timestamp);
        context.encode_field(2, self.root);
        context.encode_field(3, self.next_validators_hash);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<ConsensusState> {
        let mut consensus_state = Default::<ConsensusState>::default();
        if !context.decode_field(1, ref consensus_state.timestamp) {
            return Option::None;
        }
        if !context.decode_field(2, ref consensus_state.root) {
            return Option::None;
        }
        if !context.decode_field(3, ref consensus_state.next_validators_hash) {
            return Option::None;
        }
        Option::Some(consensus_state)
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ConsensusStateAsProtoName of ProtoName<ConsensusState> {
    fn type_url() -> ByteArray {
        "ConsensusState"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct Misbehaviour {
    client_id: ByteArray,
    header_1: Header,
    header_2: Header,
}

impl MisbehaviourAsProtoMessage of ProtoMessage<Misbehaviour> {
    fn encode_raw(self: @Misbehaviour, ref context: EncodeContext) {
        context.encode_field(1, self.client_id);
        context.encode_field(2, self.header_1);
        context.encode_field(3, self.header_2);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Misbehaviour> {
        let mut misbehaviour = Default::<Misbehaviour>::default();
        if !context.decode_field(1, ref misbehaviour.client_id) {
            return Option::None;
        }
        if !context.decode_field(2, ref misbehaviour.header_1) {
            return Option::None;
        }
        if !context.decode_field(3, ref misbehaviour.header_2) {
            return Option::None;
        }
        Option::Some(misbehaviour)
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl MisbehaviourAsProtoName of ProtoName<Misbehaviour> {
    fn type_url() -> ByteArray {
        "Misbehaviour"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct Header {
    signed_header: SignedHeader,
    validator_set: ValidatorSet,
    trusted_height: Height,
    trusted_validator_set: ValidatorSet,
}

impl HeaderAsProtoMessage of ProtoMessage<Header> {
    fn encode_raw(self: @Header, ref context: EncodeContext) {
        context.encode_field(1, self.signed_header);
        context.encode_field(2, self.validator_set);
        context.encode_field(3, self.trusted_height);
        context.encode_field(4, self.trusted_validator_set);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Header> {
        let mut header = Default::<Header>::default();
        if !context.decode_field(1, ref header.signed_header) {
            return Option::None;
        }
        if !context.decode_field(2, ref header.validator_set) {
            return Option::None;
        }
        if !context.decode_field(3, ref header.trusted_height) {
            return Option::None;
        }
        if !context.decode_field(4, ref header.trusted_validator_set) {
            return Option::None;
        }
        Option::Some(header)
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

pub impl HeaderAsProtoName of ProtoName<Header> {
    fn type_url() -> ByteArray {
        "Header"
    }
}
