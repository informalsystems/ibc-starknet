use cometbft::ibc::{Height, MerkleRoot};
use cometbft::ics23::ProofSpec;
use protobuf::types::wkt::{Timestamp, Duration};
use cometbft::utils::Fraction;
use cometbft::types::{SignedHeader, ValidatorSet};

use protobuf::types::message::{ProtoMessage, ProtoCodecImpl};
use protobuf::primitives::array::{ByteArrayAsProtoMessage, ArrayAsProtoMessage};
use protobuf::primitives::numeric::{NumberAsProtoMessage, I32AsProtoMessage, BoolAsProtoMessage};
use protobuf::types::tag::WireType;

#[derive(Default, Debug, Drop, PartialEq, Serde)]
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
    fn encode_raw(self: @ClientState, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.chain_id, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.trust_level, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(3, self.trusting_period, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(4, self.unbonding_period, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(5, self.max_clock_drift, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(6, self.frozen_height, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(7, self.latest_height, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(8, self.proof_specs, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(9, self.upgrade_path, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(10, self.allow_update_after_expiry, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(
            11, self.allow_update_after_misbehaviour, ref output
        );
    }

    fn decode_raw(ref value: ClientState, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;
        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.chain_id, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(
            2, ref value.trust_level, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            3, ref value.trusting_period, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            4, ref value.unbonding_period, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            5, ref value.max_clock_drift, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            6, ref value.frozen_height, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            7, ref value.latest_height, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            8, ref value.proof_specs, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            9, ref value.upgrade_path, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            10, ref value.allow_update_after_expiry, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            11, ref value.allow_update_after_misbehaviour, serialized, ref index
        );
        assert(index == bound, 'invalid length for ClientState');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}


#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct ConsensusState {
    timestamp: Timestamp,
    root: MerkleRoot,
    next_validators_hash: ByteArray,
}

impl ConsensusStateAsProtoMessage of ProtoMessage<ConsensusState> {
    fn encode_raw(self: @ConsensusState, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.timestamp, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.root, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(3, self.next_validators_hash, ref output);
    }

    fn decode_raw(
        ref value: ConsensusState, serialized: @ByteArray, ref index: usize, length: usize
    ) {
        let bound = index + length;
        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.timestamp, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.root, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(
            3, ref value.next_validators_hash, serialized, ref index
        );
        assert(index == bound, 'invalid length for CS');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Misbehaviour {
    client_id: ByteArray,
    header_1: Header,
    header_2: Header,
}

impl MisbehaviourAsProtoMessage of ProtoMessage<Misbehaviour> {
    fn encode_raw(self: @Misbehaviour, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.client_id, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.header_1, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(3, self.header_2, ref output);
    }

    fn decode_raw(
        ref value: Misbehaviour, serialized: @ByteArray, ref index: usize, length: usize
    ) {
        let bound = index + length;
        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.client_id, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.header_1, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(3, ref value.header_2, serialized, ref index);
        assert(index == bound, 'invalid length for Misbehaviour');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Header {
    signed_header: SignedHeader,
    validator_set: ValidatorSet,
    trusted_height: Height,
    trusted_validator_set: ValidatorSet,
}

impl HeaderAsProtoMessage of ProtoMessage<Header> {
    fn encode_raw(self: @Header, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.signed_header, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.validator_set, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(3, self.trusted_height, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(4, self.trusted_validator_set, ref output);
    }

    fn decode_raw(ref value: Header, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;
        ProtoCodecImpl::decode_length_delimited_raw(
            1, ref value.signed_header, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            2, ref value.validator_set, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            3, ref value.trusted_height, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            4, ref value.trusted_validator_set, serialized, ref index
        );
        assert(index == bound, 'invalid length for Header');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}
