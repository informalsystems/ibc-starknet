use core::marker::PhantomData;

use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::chain_components::traits::{
    CanQueryBlock, HasChannelIdType, HasCommitmentProofType, HasHeightType, HasIbcCommitmentPrefix,
    HasPacketReceiptType, HasPortIdType, HasSequenceType, PacketReceiptQuerier,
    PacketReceiptQuerierComponent,
};
use hermes_core::encoding_components::traits::{CanDecode, CanEncode, HasEncodedType, HasEncoding};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::{PortId as IbcPortId, Sequence as IbcSequence};
use ibc::core::host::types::path::{Path, ReceiptPath};
use starknet::core::types::{Felt, StorageProof};
use starknet::macros::selector;
use starknet_crypto_lib::StarknetCryptoLib;
use starknet_storage_verifier::ibc::ibc_path_to_storage_key;

use crate::traits::{
    CanCallContract, CanQueryContractAddress, CanQueryStorageProof, HasBlobType, HasSelectorType,
    HasStorageKeyType, HasStorageProofType,
};
use crate::types::{
    ChannelId, PortId as CairoPortId, Sequence, StarknetChainStatus, StarknetCommitmentProof,
};

pub struct QueryStarknetPacketReceipt;

#[cgp_provider(PacketReceiptQuerierComponent)]
impl<Chain, Counterparty, Encoding> PacketReceiptQuerier<Chain, Counterparty>
    for QueryStarknetPacketReceipt
where
    Chain: HasHeightType<Height = u64>
        + CanQueryStorageProof
        + HasStorageKeyType<StorageKey = Felt>
        + HasStorageProofType<StorageProof = StorageProof>
        + CanQueryBlock<Block = StarknetChainStatus>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = IbcPortId>
        + HasPacketReceiptType<Counterparty, PacketReceipt = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
        + HasBlobType<Blob = Vec<Felt>>
        + HasSelectorType<Selector = Felt>
        + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanCallContract
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasSequenceType<Chain, Sequence = IbcSequence>,
    Encoding: CanEncode<ViaCairo, Product![CairoPortId, ChannelId, Sequence]>
        + CanDecode<ViaCairo, bool>
        + HasEncodedType<Encoded = Vec<Felt>>,
{
    async fn query_packet_receipt(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &IbcPortId,
        sequence: &IbcSequence,
        height: &u64,
    ) -> Result<(Option<Vec<u8>>, StarknetCommitmentProof), Chain::Error> {
        let encoding = chain.encoding();

        let contract_address = chain.query_contract_address(PhantomData).await?;

        let calldata = encoding
            .encode(&product![port_id.clone(), channel_id.clone(), *sequence])
            .map_err(Chain::raise_error)?;

        let output = chain
            .call_contract(
                &contract_address,
                &selector!("packet_receipt"),
                &calldata,
                Some(height),
            )
            .await?;

        let receipt_status = encoding.decode(&output).map_err(Chain::raise_error)?;

        let receipt = if receipt_status { Some(vec![1]) } else { None };

        let block = chain.query_block(height).await?;

        let ibc_path = Path::Receipt(ReceiptPath::new(port_id, channel_id, *sequence));

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

        Ok((receipt, dummy_proof))
    }
}
