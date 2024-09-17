use core::array::TryFromSliceError;
use core::convert::Infallible;
use core::num::{ParseIntError, TryFromIntError};
use core::str::Utf8Error;

use cgp::core::error::{ErrorRaiser, ProvideErrorType};
use cgp::prelude::*;
use hermes_encoding_components::traits::convert::CanConvertBothWays;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::encode_and_decode_mut::CanEncodeAndDecodeMut;
use hermes_protobuf_encoding_components::impls::any::TypeUrlMismatchError;
use hermes_protobuf_encoding_components::impls::encode_mut::chunk::{
    InvalidWireType, UnsupportedWireType,
};
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::RequiredFieldTagNotFound;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use ibc_client_starknet_types::encoding::components::*;
use ibc_client_starknet_types::{StarknetClientState, StarknetConsensusState};
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::commitment_types::commitment::CommitmentRoot;
use ibc_core::primitives::{Timestamp, TimestampError};
use prost::DecodeError;

pub struct StarknetLightClientEncoding;

pub struct StarknetLightClientEncodingContextComponents;

impl HasComponents for StarknetLightClientEncoding {
    type Components = StarknetLightClientEncodingContextComponents;
}

with_starknet_light_client_encoding_components! {
    delegate_components! {
        StarknetLightClientEncodingContextComponents {
            @StarknetLightClientEncodingComponents: StarknetLightClientEncodingComponents
        }
    }
}

impl ProvideErrorType<StarknetLightClientEncoding>
    for StarknetLightClientEncodingContextComponents
{
    type Error = ClientError;
}

impl ErrorRaiser<StarknetLightClientEncoding, ClientError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: ClientError) -> ClientError {
        e
    }
}

impl ErrorRaiser<StarknetLightClientEncoding, Infallible>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: Infallible) -> ClientError {
        match e {}
    }
}

impl ErrorRaiser<StarknetLightClientEncoding, DecodeError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: DecodeError) -> ClientError {
        ClientError::Other {
            description: e.to_string(),
        }
    }
}

impl ErrorRaiser<StarknetLightClientEncoding, Utf8Error>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: Utf8Error) -> ClientError {
        ClientError::Other {
            description: e.to_string(),
        }
    }
}

impl ErrorRaiser<StarknetLightClientEncoding, ParseIntError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: ParseIntError) -> ClientError {
        ClientError::Other {
            description: e.to_string(),
        }
    }
}

impl ErrorRaiser<StarknetLightClientEncoding, TryFromIntError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: TryFromIntError) -> ClientError {
        ClientError::Other {
            description: e.to_string(),
        }
    }
}

impl ErrorRaiser<StarknetLightClientEncoding, TryFromSliceError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: TryFromSliceError) -> ClientError {
        ClientError::Other {
            description: e.to_string(),
        }
    }
}

impl ErrorRaiser<StarknetLightClientEncoding, TimestampError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: TimestampError) -> ClientError {
        ClientError::Other {
            description: e.to_string(),
        }
    }
}

impl ErrorRaiser<StarknetLightClientEncoding, UnsupportedWireType>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: UnsupportedWireType) -> ClientError {
        ClientError::Other {
            description: format!("{:?}", e),
        }
    }
}

impl ErrorRaiser<StarknetLightClientEncoding, InvalidWireType>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: InvalidWireType) -> ClientError {
        ClientError::Other {
            description: format!("{:?}", e),
        }
    }
}

impl ErrorRaiser<StarknetLightClientEncoding, RequiredFieldTagNotFound>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: RequiredFieldTagNotFound) -> ClientError {
        ClientError::Other {
            description: format!("{:?}", e),
        }
    }
}

impl ErrorRaiser<StarknetLightClientEncoding, TypeUrlMismatchError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: TypeUrlMismatchError) -> ClientError {
        ClientError::Other {
            description: format!("{:?}", e),
        }
    }
}

pub trait CanUseStarknetLightClientEncoding:
    CanEncodeAndDecode<ViaProtobuf, Any>
    + CanEncodeAndDecode<ViaProtobuf, Height>
    + CanEncodeAndDecode<ViaProtobuf, StarknetClientState>
    + CanEncodeAndDecode<ViaProtobuf, StarknetConsensusState>
    + CanEncodeAndDecode<ViaAny, StarknetClientState>
    + CanEncodeAndDecode<ViaAny, StarknetConsensusState>
    + CanConvertBothWays<Any, StarknetClientState>
    + CanConvertBothWays<Any, StarknetConsensusState>
    + CanEncodeAndDecodeMut<ViaProtobuf, Timestamp>
    + CanEncodeAndDecodeMut<ViaProtobuf, CommitmentRoot>
{
}

impl CanUseStarknetLightClientEncoding for StarknetLightClientEncoding {}
