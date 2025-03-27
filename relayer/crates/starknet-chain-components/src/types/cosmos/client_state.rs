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
use ibc_proto::ics23::{InnerSpec, LeafOp, ProofSpec};
use tracing::info;

use crate::types::cosmos::height::Height;

#[derive(Debug, HasField)]
pub struct CometClientState {
    pub latest_height: Height,
    pub trusting_period: Duration,
    pub unbonding_period: Duration,
    pub max_clock_drift: Duration,
    pub status: ClientStatus,
    pub chain_id: ChainId,
    pub proof_specs: ProofSpecs,
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
                EncodeField<symbol!("proof_specs"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeCometClientState {
    type From = Product![
        Height,
        Duration,
        Duration,
        Duration,
        ClientStatus,
        ChainId,
        ProofSpecs
    ];
    type To = CometClientState;

    fn transform(
        product![
            latest_height,
            trusting_period,
            unbonding_period,
            max_clock_drift,
            status,
            chain_id,
            proof_specs
        ]: Self::From,
    ) -> CometClientState {
        info!("will transform CometClientState");
        let t = CometClientState {
            latest_height,
            trusting_period,
            unbonding_period,
            max_clock_drift,
            status,
            chain_id,
            proof_specs,
        };
        info!("done: {t:#?}");
        t
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
    Encoding: CanDecodeMut<Strategy, String> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ChainId, Encoding::Error> {
        let chain_id_str = encoding.decode_mut(buffer)?;
        ChainId::new(&chain_id_str).map_err(|_| Encoding::raise_error("invalid chain id"))
    }
}

pub struct EncodeLeafOp;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, LeafOp> for EncodeLeafOp
where
    Encoding: CanEncodeMut<Strategy, Product![u32, u32, u32, u32, Vec<u8>]>,
{
    fn encode_mut(
        encoding: &Encoding,
        proof_specs: &LeafOp,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let LeafOp {
            hash,
            prehash_key,
            prehash_value,
            length,
            prefix,
        } = serde_json::to_value(proof_specs)
            .and_then(serde_json::from_value)
            .unwrap();
        //.map_err(|_| Encoding::raise_error("invalid connection end"))?;
        let hash = hash as u32;
        let prehash_key = prehash_key as u32;
        let prehash_value = prehash_value as u32;
        let length = length as u32;

        encoding.encode_mut(
            &product![hash, prehash_key, prehash_value, length, prefix,],
            buffer,
        )?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, LeafOp> for EncodeLeafOp
where
    Encoding: CanDecodeMut<Strategy, Product![u32, u32, u32, u32, Vec<u8>]>
        + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<LeafOp, Encoding::Error> {
        info!("will decode LeafOp");
        let product![hash, prehash_key, prehash_value, length, prefix] =
            encoding.decode_mut(buffer)?;
        Ok(LeafOp {
            hash: hash as i32,
            prehash_key: prehash_key as i32,
            prehash_value: prehash_value as i32,
            length: length as i32,
            prefix,
        })
    }
}

pub struct EncodeInnerSpec;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, InnerSpec> for EncodeInnerSpec
where
    Encoding: CanEncodeMut<Strategy, Product![Vec<u32>, u32, u32, u32, Vec<u8>, u32]>,
{
    fn encode_mut(
        encoding: &Encoding,
        proof_specs: &InnerSpec,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let InnerSpec {
            child_order,
            child_size,
            min_prefix_length,
            max_prefix_length,
            empty_child,
            hash,
        } = serde_json::to_value(proof_specs)
            .and_then(serde_json::from_value)
            .unwrap();
        //.map_err(|_| Encoding::raise_error("invalid connection end"))?;
        let child_order = child_order.iter().map(|x| *x as u32).collect();
        let child_size = child_size as u32;
        let min_prefix_length = min_prefix_length as u32;
        let max_prefix_length = max_prefix_length as u32;
        let hash = hash as u32;

        encoding.encode_mut(
            &product![
                child_order,
                child_size,
                min_prefix_length,
                max_prefix_length,
                empty_child,
                hash,
            ],
            buffer,
        )?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, InnerSpec> for EncodeInnerSpec
where
    Encoding: CanDecodeMut<Strategy, Product![Vec<u32>, u32, u32, u32, Vec<u8>, u32]>
        + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<InnerSpec, Encoding::Error> {
        info!("will decode InnerSpec");
        let product![
            child_order,
            child_size,
            min_prefix_length,
            max_prefix_length,
            empty_child,
            hash
        ] = encoding.decode_mut(buffer)?;
        let child_order = child_order.iter().map(|x| *x as i32).collect();
        Ok(InnerSpec {
            child_order,
            child_size: child_size as i32,
            min_prefix_length: min_prefix_length as i32,
            max_prefix_length: max_prefix_length as i32,
            empty_child,
            hash: hash as i32,
        })
    }
}

pub struct EncodeProofSpec;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ProofSpec> for EncodeProofSpec
where
    Encoding: CanEncodeMut<Strategy, Product![LeafOp, InnerSpec, u32, u32, bool]>,
{
    fn encode_mut(
        encoding: &Encoding,
        proof_specs: &ProofSpec,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let ProofSpec {
            leaf_spec,
            inner_spec,
            max_depth,
            min_depth,
            prehash_key_before_comparison,
        } = serde_json::to_value(proof_specs)
            .and_then(serde_json::from_value)
            .unwrap();
        //.map_err(|_| Encoding::raise_error("invalid connection end"))?;

        let leaf_spec = leaf_spec.unwrap();
        let inner_spec = inner_spec.unwrap();
        let max_depth = max_depth as u32;
        let min_depth = min_depth as u32;

        encoding.encode_mut(
            &product![
                leaf_spec,
                inner_spec,
                max_depth,
                min_depth,
                prehash_key_before_comparison,
            ],
            buffer,
        )?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ProofSpec> for EncodeProofSpec
where
    Encoding: CanDecodeMut<Strategy, Product![LeafOp, InnerSpec, u32, u32, bool]>
        + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ProofSpec, Encoding::Error> {
        info!("will decode ProofSpec");
        let product![
            leaf_spec,
            inner_spec,
            max_depth,
            min_depth,
            prehash_key_before_comparison
        ] = encoding.decode_mut(buffer)?;
        Ok(ProofSpec {
            leaf_spec: Some(leaf_spec),
            inner_spec: Some(inner_spec),
            max_depth: max_depth as i32,
            min_depth: min_depth as i32,
            prehash_key_before_comparison,
        })
    }
}
pub struct EncodeProofSpecs;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ProofSpecs> for EncodeProofSpecs
where
    Encoding: CanEncodeMut<Strategy, Vec<ProofSpec>>,
{
    fn encode_mut(
        encoding: &Encoding,
        proof_specs: &ProofSpecs,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        // FIXME: ibc-rs type doesn't have public fields

        #[derive(serde::Deserialize)]
        struct DummyProofSpecs(pub Vec<ProofSpec>);

        let DummyProofSpecs(specs) = serde_json::to_value(proof_specs)
            .and_then(serde_json::from_value)
            .unwrap();
        //.map_err(|_| Encoding::raise_error("invalid connection end"))?;

        encoding.encode_mut(&specs, buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ProofSpecs> for EncodeProofSpecs
where
    Encoding: CanDecodeMut<Strategy, Vec<ProofSpec>> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ProofSpecs, Encoding::Error> {
        info!("will decode ProofSpecs");
        let proof_spec_vec = encoding.decode_mut(buffer)?;
        ProofSpecs::try_from(proof_spec_vec)
            .map_err(|e| Encoding::raise_error("failed to convert Vec<ProofSpec> to ProofSpecs"))
    }
}
