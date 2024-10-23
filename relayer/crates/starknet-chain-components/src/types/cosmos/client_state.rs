use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_cairo_encoding_components::types::either::Either;
use hermes_cairo_encoding_components::Sum;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
use hermes_encoding_components::HList;
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};

use crate::types::cosmos::height::Height;

#[derive(Debug, HasField)]
pub struct CometClientState {
    pub latest_height: Height,
    pub trusting_period: u64,
    pub status: ClientStatus,
}

pub struct EncodeCometClientState;

delegate_components! {
    EncodeCometClientState {
        MutEncoderComponent: CombineEncoders<
            HList![
                EncodeField<symbol!("latest_height"), UseContext>,
                EncodeField<symbol!("trusting_period"), UseContext>,
                EncodeField<symbol!("status"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeCometClientState {
    type From = HList![Height, u64, ClientStatus];
    type To = CometClientState;

    fn transform(HList![latest_height, trusting_period, status]: Self::From) -> CometClientState {
        CometClientState {
            latest_height,
            trusting_period,
            status,
        }
    }
}

#[derive(Debug)]
pub enum ClientStatus {
    Active,
    Expired,
    Frozen(Height),
}

pub struct EncodeClientStatus;

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
