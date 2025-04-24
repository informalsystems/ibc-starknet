use core::time::Duration;

use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_chain_components::traits::types::chain_id::HasChainIdType;
use hermes_chain_components::traits::types::client_state::{
    ClientStateFieldsComponent, ClientStateFieldsGetter, ClientStateTypeComponent,
    HasClientStateType,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, MutDecoder, MutDecoderComponent,
};
use hermes_encoding_components::traits::encode_mut::{
    CanEncodeMut, MutEncoder, MutEncoderComponent,
};
use ibc::clients::tendermint::types::{
    AllowUpdate, ClientState as IbcCometClientState, TrustThreshold,
};
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height as CosmosHeight;
use ibc::core::commitment_types::specs::ProofSpecs;
use ibc::core::host::types::identifiers::ChainId;
use ibc::primitives::proto::Any;
use ibc_proto::ics23::{InnerSpec, LeafOp, ProofSpec};

use crate::types::cosmos::height::Height;

// FIXME: use ibc-rs type
#[derive(Debug, HasField, HasFields)]
pub struct CometClientState {
    pub latest_height: Height,
    pub trusting_period: Duration,
    pub unbonding_period: Duration,
    pub max_clock_drift: Duration,
    pub trust_level: TrustThreshold,
    pub status: ClientStatus,
    pub chain_id: ChainId,
    pub proof_specs: ProofSpecs,
    // as done in cairo
    pub upgrade_path: [String; 2],
}

#[derive(Debug, HasFields)]
pub enum ClientStatus {
    Active,
    Expired,
    Frozen(Height),
}

pub struct UseCometClientState;

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

impl From<CometClientState> for IbcCometClientState {
    fn from(client_state: CometClientState) -> Self {
        Self::new(
            client_state.chain_id,
            client_state.trust_level,
            client_state.trusting_period,
            client_state.unbonding_period,
            client_state.max_clock_drift,
            CosmosHeight::new(
                client_state.latest_height.revision_number,
                client_state.latest_height.revision_height,
            )
            .expect("no error"),
            client_state.proof_specs,
            client_state.upgrade_path.to_vec(),
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

pub struct EncodeTrustThreshold;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, TrustThreshold> for EncodeTrustThreshold
where
    Encoding: CanEncodeMut<Strategy, Product![u64, u64]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &TrustThreshold,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&product![value.numerator(), value.denominator()], buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, TrustThreshold> for EncodeTrustThreshold
where
    Encoding: CanDecodeMut<Strategy, Product![u64, u64]> + CanRaiseAsyncError<ClientError>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<TrustThreshold, Encoding::Error> {
        let product![numerator, denominator] = encoding.decode_mut(buffer)?;
        TrustThreshold::new(numerator, denominator).map_err(Encoding::raise_error)
    }
}

pub struct EncodeLeafOp;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, LeafOp> for EncodeLeafOp
where
    Encoding: CanEncodeMut<Strategy, Product![u32, u32, u32, u32, Vec<u8>]>
        + CanRaiseAsyncError<&'static str>,
{
    fn encode_mut(
        encoding: &Encoding,
        leaf_op: &LeafOp,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let LeafOp {
            hash,
            prehash_key,
            prehash_value,
            length,
            prefix,
        } = serde_json::to_value(leaf_op)
            .and_then(serde_json::from_value)
            .map_err(|_| Encoding::raise_error("invalid leaf op"))?;
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
    Encoding: CanEncodeMut<Strategy, Product![Vec<u32>, u32, u32, u32, Vec<u8>, u32]>
        + CanRaiseAsyncError<&'static str>,
{
    fn encode_mut(
        encoding: &Encoding,
        inner_spec: &InnerSpec,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let InnerSpec {
            child_order,
            child_size,
            min_prefix_length,
            max_prefix_length,
            empty_child,
            hash,
        } = serde_json::to_value(inner_spec)
            .and_then(serde_json::from_value)
            .map_err(|_| Encoding::raise_error("invalid inner spec"))?;
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
    Encoding: CanEncodeMut<Strategy, Product![LeafOp, InnerSpec, u32, u32, bool]>
        + CanRaiseAsyncError<&'static str>,
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
            .map_err(|_| Encoding::raise_error("invalid proof spec"))?;

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
    Encoding: CanEncodeMut<Strategy, Vec<ProofSpec>> + CanRaiseAsyncError<&'static str>,
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
            .map_err(|_| Encoding::raise_error("invalid proof specs"))?;

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
        let proof_spec_vec = encoding.decode_mut(buffer)?;
        ProofSpecs::try_from(proof_spec_vec)
            .map_err(|e| Encoding::raise_error("failed to convert Vec<ProofSpec> to ProofSpecs"))
    }
}
