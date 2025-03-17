use core::time::Duration;

use cgp::core::component::UseContext;
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_chain_components::traits::types::chain_id::HasChainIdType;
use hermes_chain_components::traits::types::client_state::{
    ClientStateFieldsComponent, ClientStateFieldsGetter, ClientStateTypeComponent,
    HasClientStateType,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, MutDecoder, MutDecoderComponent,
};
use hermes_encoding_components::traits::encode_mut::{
    CanEncodeMut, MutEncoder, MutEncoderComponent,
};
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
use ibc::clients::tendermint::types::{
    AllowUpdate, ClientState as IbcCometClientState, TrustThreshold,
};
use ibc::core::client::types::Height as CosmosHeight;
use ibc::core::commitment_types::specs::ProofSpecs;
use ibc::core::host::types::identifiers::ChainId;
use ibc::primitives::proto::Any;

use crate::types::cosmos::height::Height;

#[derive(Debug, HasField)]
pub struct CometClientState {
    pub latest_height: Height,
    pub trusting_period: Duration,
    pub unbonding_period: Duration,
    pub max_clock_drift: Duration,
    pub status: ClientStatus,
    pub chain_id: ChainId,
}

#[derive(Debug)]
pub enum ClientStatus {
    Active,
    Expired,
    Frozen(Height),
}

pub struct UseCometClientState;

pub struct EncodeCometClientState;

pub struct EncodeClientStatus;

delegate_components! {
    UseCometClientState {
        ClientStateTypeComponent:
            WithType<CometClientState>,
    }
}

#[cgp_provider(ClientStateFieldsComponent)]
impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty> for UseCometClientState
where
    Chain: HasClientStateType<Counterparty, ClientState = CometClientState>
        + HasHeightType<Height = CosmosHeight>
        + HasChainIdType<ChainId = ChainId>,
{
    fn client_state_latest_height(client_state: &CometClientState) -> CosmosHeight {
        CosmosHeight::new(
            client_state.latest_height.revision_number,
            client_state.latest_height.revision_height,
        )
        .unwrap()
    }

    fn client_state_is_frozen(_client_state: &CometClientState) -> bool {
        false // todo
    }

    fn client_state_has_expired(_client_state: &CometClientState, _elapsed: Duration) -> bool {
        false // todo
    }

    fn client_state_chain_id(client_state: &CometClientState) -> ChainId {
        client_state.chain_id.clone()
    }
}

delegate_components! {
    EncodeCometClientState {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("latest_height"), UseContext>,
                EncodeField<symbol!("trusting_period"), UseContext>,
                EncodeField<symbol!("unbonding_period"), UseContext>,
                EncodeField<symbol!("max_clock_drift"), UseContext>,
                EncodeField<symbol!("status"), UseContext>,
                EncodeField<symbol!("chain_id"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeCometClientState {
    type From = Product![Height, Duration, Duration, Duration, ClientStatus, ChainId];
    type To = CometClientState;

    fn transform(
        product![
            latest_height,
            trusting_period,
            unbonding_period,
            max_clock_drift,
            status,
            chain_id
        ]: Self::From,
    ) -> CometClientState {
        CometClientState {
            latest_height,
            trusting_period,
            unbonding_period,
            max_clock_drift,
            status,
            chain_id,
        }
    }
}

delegate_components! {
    EncodeClientStatus {
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]: EncodeVariantFrom<Self>,
    }
}

impl TransformerRef for EncodeClientStatus {
    type From = ClientStatus;
    type To<'a> = Sum![(), (), &'a Height];

    fn transform<'a>(from: &'a ClientStatus) -> Self::To<'a> {
        match from {
            ClientStatus::Active => Either::Left(()),
            ClientStatus::Expired => Either::Right(Either::Left(())),
            ClientStatus::Frozen(height) => Either::Right(Either::Right(Either::Left(height))),
        }
    }
}

impl Transformer for EncodeClientStatus {
    type From = Sum![(), (), Height];
    type To = ClientStatus;

    fn transform(value: Self::From) -> ClientStatus {
        match value {
            Either::Left(()) => ClientStatus::Active,
            Either::Right(Either::Left(())) => ClientStatus::Expired,
            Either::Right(Either::Right(Either::Left(height))) => ClientStatus::Frozen(height),
            Either::Right(Either::Right(Either::Right(v))) => match v {},
        }
    }
}

impl From<CometClientState> for IbcCometClientState {
    fn from(client_state: CometClientState) -> Self {
        Self::new(
            client_state.chain_id,
            TrustThreshold::ONE_THIRD,
            client_state.trusting_period,
            client_state.unbonding_period,
            client_state.max_clock_drift,
            CosmosHeight::new(
                client_state.latest_height.revision_number,
                client_state.latest_height.revision_height,
            )
            .expect("no error"),
            ProofSpecs::cosmos(),
            Vec::new(),
            AllowUpdate {
                after_expiry: false,
                after_misbehaviour: false,
            },
        )
        .expect("no error")
    }
}

impl From<CometClientState> for Any {
    fn from(client_state: CometClientState) -> Self {
        IbcCometClientState::from(client_state).into()
    }
}

pub struct EncodeChainId;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ChainId> for EncodeChainId
where
    Encoding: CanEncodeMut<Strategy, String>,
{
    fn encode_mut(
        encoding: &Encoding,
        chain_id: &ChainId,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let chain_id_str = chain_id.as_str().to_string();
        encoding.encode_mut(&chain_id_str, buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ChainId> for EncodeChainId
where
    Encoding: CanDecodeMut<Strategy, String> + CanRaiseAsyncError<String>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ChainId, Encoding::Error> {
        let chain_id_str = encoding.decode_mut(buffer)?;
        ChainId::new(&chain_id_str)
            .map_err(|_| Encoding::raise_error(format!("invalid chain id: {chain_id_str}")))
    }
}
