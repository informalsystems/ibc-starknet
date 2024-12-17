use core::time::Duration;

use cgp::core::component::UseContext;
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_chain_components::traits::types::chain_id::HasChainIdType;
use hermes_chain_components::traits::types::client_state::{
    ClientStateFieldsGetter, HasClientStateType,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_cosmos_chain_components::components::client::ClientStateTypeComponent;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
use ibc::core::client::types::Height as CosmosHeight;
use ibc::core::host::types::identifiers::ChainId;

use crate::types::cosmos::height::Height;

#[derive(Debug, HasField)]
pub struct CometClientState {
    pub latest_height: Height,
    pub trusting_period: u64,
    pub status: ClientStatus,
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

    fn client_state_chain_id(_client_state: &CometClientState) -> ChainId {
        // FIXME: Store Cosmos chain ID in CometClientState and return it here
        ChainId::new("cosmos").unwrap()
    }
}

delegate_components! {
    EncodeCometClientState {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("latest_height"), UseContext>,
                EncodeField<symbol!("trusting_period"), UseContext>,
                EncodeField<symbol!("status"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeCometClientState {
    type From = Product![Height, u64, ClientStatus];
    type To = CometClientState;

    fn transform(product![latest_height, trusting_period, status]: Self::From) -> CometClientState {
        CometClientState {
            latest_height,
            trusting_period,
            status,
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
