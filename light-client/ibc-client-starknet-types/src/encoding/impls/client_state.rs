use core::str::FromStr;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cosmos_encoding_components::impls::chain_id::EncodeChainIdField;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use hermes_protobuf_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::DecodeRequiredProtoField;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::encode::EncodeLengthDelimitedProtoField;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::string::EncodeStringField;
use ibc_core::client::types::Height;
use ibc_core::host::types::identifiers::ChainId;
use secp256k1::{Error as Secp256k1Error, PublicKey};

use crate::StarknetClientState;

pub struct EncodeStarknetClientState;

delegate_components! {
    EncodeStarknetClientState {
        MutEncoderComponent:
            CombineEncoders<Product![
                EncodeField<
                    symbol!("latest_height"),
                    EncodeLengthDelimitedProtoField<1, UseContext>,
                >,
                EncodeField<
                    symbol!("chain_id"),
                    EncodeChainIdField<2>,
                >,
                EncodeField<
                    symbol!("pub_key"),
                    EncodePublicKeyField<3>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<Product![
                DecodeRequiredProtoField<1, UseContext>,
                EncodeChainIdField<2>,
                EncodePublicKeyField<3>,
            ]>
        >,
    }
}

impl Transformer for EncodeStarknetClientState {
    type From = Product![Height, ChainId, PublicKey];

    type To = StarknetClientState;

    fn transform(product![latest_height, chain_id, pub_key]: Self::From) -> Self::To {
        StarknetClientState {
            latest_height,
            chain_id,
            pub_key,
        }
    }
}

// TODO: Move to Hermes SDK
pub struct EncodePublicKeyField<const TAG: u32>;

impl<Encoding, Strategy, const TAG: u32> MutEncoder<Encoding, Strategy, PublicKey>
    for EncodePublicKeyField<TAG>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    EncodeStringField<TAG>: MutEncoder<Encoding, Strategy, String>,
{
    fn encode_mut(
        encoding: &Encoding,
        pub_key: &PublicKey,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        <EncodeStringField<TAG>>::encode_mut(encoding, &pub_key.to_string(), buffer)?;

        Ok(())
    }
}

impl<Encoding, Strategy, const TAG: u32> MutDecoder<Encoding, Strategy, PublicKey>
    for EncodePublicKeyField<TAG>
where
    Encoding: HasDecodeBufferType + CanRaiseAsyncError<Secp256k1Error>,
    EncodeStringField<TAG>: MutDecoder<Encoding, Strategy, String>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<PublicKey, Encoding::Error> {
        let pub_key_str = <EncodeStringField<TAG>>::decode_mut(encoding, buffer)?;

        PublicKey::from_str(&pub_key_str).map_err(Encoding::raise_error)
    }
}
