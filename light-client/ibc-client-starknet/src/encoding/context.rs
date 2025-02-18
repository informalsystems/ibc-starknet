use core::array::TryFromSliceError;
use core::convert::Infallible;
use core::num::{ParseIntError, TryFromIntError};
use core::str::Utf8Error;

use cgp::core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_encoding_components::traits::convert::{CanConvert, CanConvertBothWays};
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
use ibc_client_starknet_types::header::{SignedStarknetHeader, StarknetHeader};
use ibc_client_starknet_types::{StarknetClientState, StarknetConsensusState};
use ibc_core::client::types::error::ClientError;
use ibc_core::client::types::Height;
use ibc_core::commitment_types::commitment::CommitmentRoot;
use ibc_core::host::types::error::{DecodingError, IdentifierError};
use ibc_core::primitives::{Timestamp, TimestampError};
use prost::DecodeError;

#[cgp_context(StarknetLightClientEncodingContextComponents: StarknetLightClientEncodingComponents)]
pub struct StarknetLightClientEncoding;

#[cgp_provider(ErrorTypeProviderComponent)]
impl ErrorTypeProvider<StarknetLightClientEncoding>
    for StarknetLightClientEncodingContextComponents
{
    type Error = ClientError;
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, ClientError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: ClientError) -> ClientError {
        e
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, Infallible>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: Infallible) -> ClientError {
        match e {}
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, DecodeError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: DecodeError) -> ClientError {
        ClientError::ClientSpecific {
            description: e.to_string(),
        }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, Utf8Error>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: Utf8Error) -> ClientError {
        ClientError::ClientSpecific {
            description: e.to_string(),
        }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, ParseIntError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: ParseIntError) -> ClientError {
        ClientError::ClientSpecific {
            description: e.to_string(),
        }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, TryFromIntError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: TryFromIntError) -> ClientError {
        ClientError::ClientSpecific {
            description: e.to_string(),
        }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, TryFromSliceError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: TryFromSliceError) -> ClientError {
        ClientError::ClientSpecific {
            description: e.to_string(),
        }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, TimestampError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: TimestampError) -> ClientError {
        ClientError::ClientSpecific {
            description: e.to_string(),
        }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, DecodingError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: DecodingError) -> ClientError {
        ClientError::ClientSpecific {
            description: e.to_string(),
        }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, UnsupportedWireType>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: UnsupportedWireType) -> ClientError {
        ClientError::ClientSpecific {
            description: format!("{e:?}"),
        }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, InvalidWireType>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: InvalidWireType) -> ClientError {
        ClientError::ClientSpecific {
            description: format!("{e:?}"),
        }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, RequiredFieldTagNotFound>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: RequiredFieldTagNotFound) -> ClientError {
        ClientError::ClientSpecific {
            description: format!("{e:?}"),
        }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, TypeUrlMismatchError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: TypeUrlMismatchError) -> ClientError {
        ClientError::ClientSpecific {
            description: format!("{e:?}"),
        }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<StarknetLightClientEncoding, IdentifierError>
    for StarknetLightClientEncodingContextComponents
{
    fn raise_error(e: IdentifierError) -> ClientError {
        ClientError::ClientSpecific {
            description: format!("{e:?}"),
        }
    }
}

pub trait CanUseStarknetLightClientEncoding:
    Async
    + CanEncodeAndDecode<ViaProtobuf, Any>
    + CanEncodeAndDecode<ViaProtobuf, Height>
    + CanEncodeAndDecode<ViaProtobuf, StarknetClientState>
    + CanEncodeAndDecode<ViaProtobuf, StarknetConsensusState>
    + CanEncodeAndDecode<ViaProtobuf, StarknetHeader>
    + CanEncodeAndDecode<ViaProtobuf, SignedStarknetHeader>
    + CanEncodeAndDecode<ViaAny, StarknetClientState>
    + CanEncodeAndDecode<ViaAny, StarknetConsensusState>
    + CanEncodeAndDecode<ViaAny, StarknetHeader>
    + CanEncodeAndDecode<ViaAny, SignedStarknetHeader>
    + CanConvertBothWays<Any, StarknetClientState>
    + CanConvertBothWays<Any, StarknetConsensusState>
    + CanConvertBothWays<Any, StarknetHeader>
    + CanConvertBothWays<Any, SignedStarknetHeader>
    + CanConvert<StarknetHeader, Any>
    + CanConvert<SignedStarknetHeader, Any>
    + CanEncodeAndDecodeMut<ViaProtobuf, Timestamp>
    + CanEncodeAndDecodeMut<ViaProtobuf, CommitmentRoot>
{
}

impl CanUseStarknetLightClientEncoding for StarknetLightClientEncoding {}
