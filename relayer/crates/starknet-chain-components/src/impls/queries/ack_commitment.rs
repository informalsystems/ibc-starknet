use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, HasAckCommitmentHashType, HasChannelIdType, HasCommitmentProofType,
    HasHeightType, HasIbcCommitmentPrefix, HasPortIdType, HasSequenceType,
    PacketAckCommitmentQuerier, PacketAckCommitmentQuerierComponent,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_cosmos_core::chain_components::types::Secp256k1KeyPair;
use hermes_prelude::*;
use ibc::core::host::types::identifiers::{PortId as IbcPortId, Sequence as IbcSequence};
use ibc::core::host::types::path::{AckPath, Path};
use starknet::core::types::Felt;
use starknet::macros::selector;
use starknet_crypto_lib::StarknetCryptoLib;
use starknet_storage_verifier::ibc::ibc_path_to_storage_key;
use starknet_v14::core::types::StorageProof;

use crate::traits::{
    CanCallContract, CanQueryContractAddress, CanQueryStorageProof, HasBlobType, HasSelectorType,
    HasStarknetProofSigner, HasStorageKeyType, HasStorageProofType,
};
use crate::types::{
    ChannelId, PortId as CairoPortId, Sequence, StarknetChainStatus, StarknetCommitmentProof,
};
pub struct QueryStarknetAckCommitment;

#[cgp_provider(PacketAckCommitmentQuerierComponent)]
impl<Chain, Counterparty, Encoding> PacketAckCommitmentQuerier<Chain, Counterparty>
    for QueryStarknetAckCommitment
where
    Chain: HasHeightType<Height = u64>
        + CanQueryStorageProof
        + HasStorageKeyType<StorageKey = Felt>
        + HasStorageProofType<StorageProof = StorageProof>
        + CanQueryBlock<Block = StarknetChainStatus>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasAckCommitmentHashType<AckCommitmentHash = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasSequenceType<Chain, Sequence = IbcSequence>,

    Encoding: CanEncode<ViaCairo, Product![CairoPortId, ChannelId, Sequence]>
        + CanDecode<ViaCairo, Product![[u32; 8]]>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_packet_ack_commitment_with_proof(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &IbcPortId,
        sequence: &IbcSequence,
        height: &u64,
    ) -> Result<(Vec<u8>, StarknetCommitmentProof), Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding
            .encode(&product![port_id.clone(), channel_id.clone(), *sequence])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("packet_acknowledgement"),
                &calldata,
                Some(height),
            )
            .await?;

        let product![ack,] = encoding.decode(&output).map_err(Chain::raise_error)?;

        let ack_bytes = ack
            .into_iter()
            // TODO(rano): cairo uses [u32; 8], but in cosmos it's Vec<u8>
            .flat_map(|felt| felt.to_be_bytes())
            .collect::<Vec<_>>();

        let block = chain.query_block(height).await?;

        let ibc_path = Path::Ack(AckPath::new(port_id, channel_id, *sequence));

        let felt_path: Felt = ibc_path_to_storage_key(&StarknetCryptoLib, ibc_path);

        // key == path
        let storage_proof: StorageProof = chain
            .query_storage_proof(height, &contract_address, &[felt_path])
            .await?;

        let storage_proof_bytes = serde_json::to_vec(&storage_proof).map_err(Chain::raise_error)?;

        let dummy_proof = StarknetCommitmentProof {
            proof_height: block.height,
            proof_bytes: storage_proof_bytes,
        };

        // `ack_bytes` is stored after hashing.
        // query block event is required to get the original ack_bytes.

        Ok((ack_bytes, dummy_proof))
    }
}
