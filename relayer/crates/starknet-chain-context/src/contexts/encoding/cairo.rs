use core::iter::Peekable;
use core::slice::Iter;
use core::time::Duration;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::encoding_components::impls::GetDefaultEncoding;
use hermes_core::encoding_components::traits::{
    CanDecode, CanEncode, CanEncodeAndDecode, CanPeekDecodeBuffer, DefaultEncodingGetter,
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeProviderComponent,
    HasDecodeBufferType, HasEncodeBufferType, HasEncodedType, HasEncodingType,
};
use hermes_cosmos::error::impls::UseHermesError;
use hermes_cosmos::error::types::HermesError;
use hermes_prelude::*;
use hermes_starknet_chain_components::components::*;
use hermes_starknet_chain_components::types::{
    ChannelEnd, ChannelId, ClientId, ClientStatus, CometClientState, CometConsensusState,
    ConnectionEnd, ConnectionId, CreateClientResponse, Denom, DeployErc20TokenMessage, Height,
    MsgChanOpenAck, MsgChanOpenConfirm, MsgChanOpenInit, MsgChanOpenTry, MsgConnOpenAck,
    MsgConnOpenConfirm, MsgConnOpenInit, MsgConnOpenTry, MsgRegisterApp, MsgRegisterClient, Packet,
    Participant, PrefixedDenom, Sequence, TracePrefix, TransferErc20TokenMessage,
    TransferPacketData,
};
use ibc::clients::tendermint::types::Header as TendermintLcHeader;
use starknet::core::types::{Felt, U256};

use crate::impls::HandleStarknetChainError;

#[cgp_context(StarknetCairoEncodingContextComponents: StarknetCairoEncodingComponents)]
pub struct StarknetCairoEncoding;

pub struct StarknetEncodeMutComponents;

delegate_components! {
    StarknetCairoEncodingContextComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
    }
}

pub struct UseStarknetCairoEncoding;

delegate_components! {
    UseStarknetCairoEncoding {
        EncodingTypeProviderComponent<AsFelt>:
            UseType<StarknetCairoEncoding>,
        EncodingGetterComponent<AsFelt>:
            GetDefaultEncoding,
    }
}

#[cgp_provider(DefaultEncodingGetterComponent<AsFelt>)]
impl<Context> DefaultEncodingGetter<Context, AsFelt> for UseStarknetCairoEncoding
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
    + CanEncode<ViaCairo, TendermintLcHeader>
    + CanDecode<ViaCairo, CreateClientResponse>
{
}

impl CanUseCairoEncoding for StarknetCairoEncoding {}
