use core::iter::Peekable;
use core::slice::Iter;
use core::time::Duration;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::decode_mut::CanPeekDecodeBuffer;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, DefaultEncodingGetterComponent, EncodingGetterComponent,
    EncodingTypeComponent, HasEncodingType, ProvideEncodingType,
};
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_error::impls::UseHermesError;
use hermes_error::types::HermesError;
use hermes_starknet_chain_components::components::encoding::cairo::*;
use hermes_starknet_chain_components::types::channel_id::{ChannelEnd, ChannelId};
use hermes_starknet_chain_components::types::client_id::ClientId;
use hermes_starknet_chain_components::types::connection_id::{ConnectionEnd, ConnectionId};
use hermes_starknet_chain_components::types::cosmos::client_state::{
    ClientStatus, CometClientState,
};
use hermes_starknet_chain_components::types::cosmos::consensus_state::CometConsensusState;
use hermes_starknet_chain_components::types::cosmos::height::Height;
use hermes_starknet_chain_components::types::cosmos::update::CometUpdateHeader;
use hermes_starknet_chain_components::types::message_responses::create_client::CreateClientResponse;
use hermes_starknet_chain_components::types::messages::erc20::deploy::DeployErc20TokenMessage;
use hermes_starknet_chain_components::types::messages::erc20::transfer::TransferErc20TokenMessage;
use hermes_starknet_chain_components::types::messages::ibc::channel::{
    MsgChanOpenAck, MsgChanOpenConfirm, MsgChanOpenInit, MsgChanOpenTry,
};
use hermes_starknet_chain_components::types::messages::ibc::connection::{
    MsgConnOpenAck, MsgConnOpenConfirm, MsgConnOpenInit, MsgConnOpenTry,
};
use hermes_starknet_chain_components::types::messages::ibc::denom::{
    Denom, PrefixedDenom, TracePrefix,
};
use hermes_starknet_chain_components::types::messages::ibc::ibc_transfer::{
    Participant, TransferPacketData,
};
use hermes_starknet_chain_components::types::messages::ibc::packet::{Packet, Sequence};
use hermes_starknet_chain_components::types::register::{MsgRegisterApp, MsgRegisterClient};
use starknet::core::types::{Felt, U256};

use crate::impls::error::HandleStarknetChainError;

#[cgp_context(StarknetCairoEncodingContextComponents: StarknetCairoEncodingComponents)]
pub struct StarknetCairoEncoding;

pub struct StarknetEncodeMutComponents;

delegate_components! {
    StarknetCairoEncodingContextComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
    }
}

pub struct ProvideCairoEncoding;

delegate_components! {
    ProvideCairoEncoding {
        EncodingGetterComponent: GetDefaultEncoding,
    }
}

#[cgp_provider(EncodingTypeComponent)]
impl<Context> ProvideEncodingType<Context, AsFelt> for ProvideCairoEncoding
where
    Context: Async,
{
    type Encoding = StarknetCairoEncoding;
}

#[cgp_provider(DefaultEncodingGetterComponent)]
impl<Context> DefaultEncodingGetter<Context, AsFelt> for ProvideCairoEncoding
where
    Context: HasEncodingType<AsFelt, Encoding = StarknetCairoEncoding>,
{
    fn default_encoding() -> &'static StarknetCairoEncoding {
        &StarknetCairoEncoding
    }
}

pub trait CanUseCairoEncoding:
    HasAsyncErrorType<Error = HermesError>
    + HasEncodedType<Encoded = Vec<Felt>>
    + HasEncodeBufferType<EncodeBuffer = Vec<Felt>>
    + for<'a> HasDecodeBufferType<DecodeBuffer<'a> = Peekable<Iter<'a, Felt>>>
    + CanPeekDecodeBuffer<Felt>
    + CanEncodeAndDecode<ViaCairo, ()>
    + CanEncodeAndDecode<ViaCairo, Nil>
    + CanEncodeAndDecode<ViaCairo, Felt>
    + CanEncodeAndDecode<ViaCairo, u128>
    + CanEncodeAndDecode<ViaCairo, U256>
    + CanEncodeAndDecode<ViaCairo, u64>
    + CanEncodeAndDecode<ViaCairo, usize>
    + CanEncodeAndDecode<ViaCairo, Vec<u8>>
    + CanEncodeAndDecode<ViaCairo, Vec<Felt>>
    + CanEncodeAndDecode<ViaCairo, String>
    + CanEncode<ViaCairo, TransferErc20TokenMessage>
    + CanEncode<ViaCairo, DeployErc20TokenMessage>
    + CanEncodeAndDecode<ViaCairo, Option<String>>
    + for<'a> CanEncode<ViaCairo, &'a String>
    + CanEncodeAndDecode<ViaCairo, Denom>
    + CanEncodeAndDecode<ViaCairo, PrefixedDenom>
    + CanEncodeAndDecode<ViaCairo, TracePrefix>
    + CanEncodeAndDecode<ViaCairo, Vec<TracePrefix>>
    + CanEncodeAndDecode<ViaCairo, Participant>
    + CanEncodeAndDecode<ViaCairo, TransferPacketData>
    + CanEncodeAndDecode<ViaCairo, Height>
    + CanEncodeAndDecode<ViaCairo, Packet>
    + CanEncodeAndDecode<ViaCairo, ClientStatus>
    + CanEncodeAndDecode<ViaCairo, CometClientState>
    + CanEncodeAndDecode<ViaCairo, CometConsensusState>
    + CanEncodeAndDecode<ViaCairo, ClientId>
    + CanEncodeAndDecode<ViaCairo, ConnectionId>
    + CanEncodeAndDecode<ViaCairo, ConnectionEnd>
    + CanEncodeAndDecode<ViaCairo, ChannelId>
    + CanEncodeAndDecode<ViaCairo, ChannelEnd>
    + CanEncodeAndDecode<ViaCairo, Sequence>
    + CanEncodeAndDecode<ViaCairo, MsgRegisterClient>
    + CanEncodeAndDecode<ViaCairo, MsgRegisterApp>
    + CanEncodeAndDecode<ViaCairo, MsgConnOpenInit>
    + CanEncodeAndDecode<ViaCairo, MsgConnOpenTry>
    + CanEncodeAndDecode<ViaCairo, MsgConnOpenAck>
    + CanEncodeAndDecode<ViaCairo, MsgConnOpenConfirm>
    + CanEncodeAndDecode<ViaCairo, MsgChanOpenInit>
    + CanEncodeAndDecode<ViaCairo, MsgChanOpenTry>
    + CanEncodeAndDecode<ViaCairo, MsgChanOpenAck>
    + CanEncodeAndDecode<ViaCairo, MsgChanOpenConfirm>
    + CanEncodeAndDecode<ViaCairo, Duration>
    + CanEncode<ViaCairo, CometUpdateHeader>
    + CanDecode<ViaCairo, CreateClientResponse>
{
}

impl CanUseCairoEncoding for StarknetCairoEncoding {}
