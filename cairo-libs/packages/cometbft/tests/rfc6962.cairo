use cometbft::utils::MerkleHashImpl;
use ibc_utils::bytes::{ByteArrayIntoArrayU8, SpanU32IntoArrayU8};
use ibc_utils::hex;
use protobuf::types::message::ProtoCodecImpl;

// copied from:
// https://github.com/informalsystems/tendermint-rs/blob/6cc391c80ae988615508bd87285571ba130b604c/tendermint/src/merkle.rs#L144

#[test]
fn test_rfc6962_empty_tree() {
    let empty_tree_root_hex = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    let empty_tree_root = hex::decode_byte_array(empty_tree_root_hex);
    let empty_tree: Array<Span<u8>> = array![];

    let root = MerkleHashImpl::hash_byte_vectors(empty_tree.span());
    assert_eq!(empty_tree_root, root.span().into());
}

#[test]
fn test_rfc6962_leaf() {
    let leaf_root_hex = "395aa064aa4c29f7010acfe3f25db9485bbd4b91897b6ad7ad547639252b4d56";
    let leaf_string = "L123456";

    let leaf_root = hex::decode_byte_array(leaf_root_hex);
    let leaf_tree = array![ByteArrayIntoArrayU8::into(leaf_string).span()];

    let root = MerkleHashImpl::hash_byte_vectors(leaf_tree.span());
    assert_eq!(leaf_root, root.span().into())
}

#[test]
fn test_rfc6962_tree_of_2() {
    let node_hash_hex = "dc9a0536ff2e196d5a628a5bf377ab247bbddf83342be39699461c1e766e6646";
    let left = "N123";
    let right = "N456";

    let node_hash = hex::decode_byte_array(node_hash_hex);
    let tree = array![
        ByteArrayIntoArrayU8::into(left).span(), ByteArrayIntoArrayU8::into(right).span(),
    ];

    let root = MerkleHashImpl::hash_byte_vectors(tree.span());
    assert_eq!(node_hash, root.span().into());
}
