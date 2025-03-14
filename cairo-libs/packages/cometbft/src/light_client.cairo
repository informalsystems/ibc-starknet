use cometbft::ibc::{Height, MerkleRoot};
use cometbft::types::{SignedHeader, ValidatorSet};
use cometbft::utils::Fraction;
use ics23::ProofSpec;
use protobuf::primitives::array::ByteArrayAsProtoMessage;
use protobuf::primitives::numeric::{BoolAsProtoMessage, I32AsProtoMessage};
use protobuf::types::message::{
    DecodeContext, DecodeContextImpl, EncodeContext, EncodeContextImpl, ProtoCodecImpl,
    ProtoMessage, ProtoName,
};
use protobuf::types::tag::WireType;
use protobuf::types::wkt::{Duration, Timestamp};

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
        let chain_id = context.decode_field(1)?;
        let trust_level = context.decode_field(2)?;
        let trusting_period = context.decode_field(3)?;
        let unbonding_period = context.decode_field(4)?;
        let max_clock_drift = context.decode_field(5)?;
        let frozen_height = context.decode_field(6)?;
        let latest_height = context.decode_field(7)?;
        let proof_specs = context.decode_repeated_field(8)?;
        let upgrade_path = context.decode_repeated_field(9)?;
        let allow_update_after_expiry = context.decode_field(10)?;
        let allow_update_after_misbehaviour = context.decode_field(11)?;
        Option::Some(
            ClientState {
                chain_id,
                trust_level,
                trusting_period,
                unbonding_period,
                max_clock_drift,
                frozen_height,
                latest_height,
                proof_specs,
                upgrade_path,
                allow_update_after_expiry,
                allow_update_after_misbehaviour,
            },
        )
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
        let timestamp = context.decode_field(1)?;
        let root = context.decode_field(2)?;
        let next_validators_hash = context.decode_field(3)?;
        Option::Some(ConsensusState { timestamp, root, next_validators_hash })
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
        let client_id = context.decode_field(1)?;
        let header_1 = context.decode_field(2)?;
        let header_2 = context.decode_field(3)?;
        Option::Some(Misbehaviour { client_id, header_1, header_2 })
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
        let signed_header = context.decode_field(1)?;
        let validator_set = context.decode_field(2)?;
        let trusted_height = context.decode_field(3)?;
        let trusted_validator_set = context.decode_field(4)?;
        Option::Some(Header { signed_header, validator_set, trusted_height, trusted_validator_set })
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
