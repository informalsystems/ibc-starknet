# ADR 001: Starknet Light Client

## Changelog

- 2025-06-25: Initial version.
- 2025-07-17: Revision with the working implementation.

## Context

We are implementing IBC protocol between Starknet and Cosmos-SDK chains. For
this, we need a light client of Starknet that can verify the validity of
Starknet block headers and the vector commitments of the committed IBC
key-values.

There are mainly two components to an IBC light client:

1. Verifying an untrusted vector commitment root to be trusted.
   - Conventionally, this is done by signature verification. This can be
     multi-sig or validator set signature.
2. Verify the membership of an IBC key-value against the trusted vector
   commitment root.
   - Conventionally, this is done by Merkle tree membership proof verification
     against the trusted vector commitment root.

## Decision

Starknet centralized sequencer generates block headers along with signatures.
The latest version of Starknet now supports storage proofs against a block.
These two features enable us to build an IBC light client for Starknet:

1. Verify the block header signature against the centralized sequencer's public
   key.
2. Verify the storage proof against the state root present in the trusted block
   header.
   1. Verify the computed state root hash from global contract and class storage
      root match.
   2. Verify the contract storage root against the global contract root using a
      Merkle proof.
   3. Verify the IBC contract key-value against the contract storage root using
      a Merkle proof.

We implement the IBC protocol in a Starknet contract (written in Cairo). So, we
only need to track the storage root of that particular deployed contract, not
necessarily the global state root of Starknet. Thus, we perform 1, 2a, 2b in
client update and discard the block header along with its state root and
maintain only the storage root of the IBC contract. This way, when we verify
multiple IBC packets, we can avoid re-performing 2a. and 2b. for each of the
packets.

### Data Types

Starknet ClientState stores the latest height, chain identifier, the public key
of the centralized sequencer and the contract address of the IBC contract. The
chain identifier, public key and the contract address must remain immutable for
the normal lifetime of the client state.

They can only be updated by a ClientUpgrade.

```rust
pub struct ClientState {
    pub latest_height: Height,
    pub chain_id: ChainId,
    pub sequencer_public_key: Felt,
    pub ibc_contract_address: Felt,
}
```

Starknet ConsensusState stores the trusted IBC contract root and the timestamp
of the latest block header.

```rust
pub struct ConsensusState {
    pub ibc_contract_root: Felt,
    pub timestamp: u64,
}
```

Starknet Header is a combination of the block header, the sequencer's signature,
and the storage proof for the IBC contract. The storage proof contains the
global class and contract roots, and the contract storage proof for the IBC
contract storage root.

```rust
pub struct Header {
    pub block_header: BlockHeader,
    pub block_signature: Signature,
    pub storage_proof: StorageProof,
}
```

### Verifying Block Header and IBC Contract Storage Root

```rust
fn verify_header(
    client_state: ClientState, // trusted; given by the ibc module

    header: Header, // untrusted; given by the relayer
) {
    let Header {
        block_header,
        block_signature,
        storage_proof,
    } = header;

    let ClientState {
        sequencer_public_key,
        ibc_contract_address,
        ...
    } = client_state;

    assert!(starknet_block_header.verify_signature(block_signature, sequencer_public_key));

    // header is now trusted

    let expected_state_root = starknet_block_header.state_root;

    let (global_class_root, global_contract_root) = storage_proof.global_roots();

    let actual_state_root = compute_global_state_root(global_class_root, global_contract_root);

    assert_eq!(
        expected_state_root, actual_state_root,
        "State root mismatch"
    );

    // contract_trie_root is now trusted

    let contract_proof = storage_proof.contract_proof();
    let ibc_contract_root = storage_proof.contract_root();

    assert!(verify_starknet_merkle_proof(
      contract_proof,
      global_contract_root,
      ibc_contract_address,
      ibc_contract_root
    ));

    // `ibc_contract_root` is now trusted and can be stored in `ConsensusState`
}
```

### Verifying Membership proof of IBC Key-Value

```rust
fn verify_membership(
    ibc_contract_root: Felt, // trusted; given by the ibc module
    key: Felt, // trusted; given by the ibc module

    // if value is None, we are checking for non-membership
    value: Option<Felt>, // untrusted; given by the relayer
    storage_proof: StorageProof, // untrusted; given by the relayer
) {
    let membership_proof = storage_proot.membership_proof();

    assert!(verify_starknet_merkle_proof(
      membership_proof,
      ibc_contract_root,
      key,
      value
    ));

    // value is now trusted
}
```

## Status

Accepted

## Consequences

### Positive

By storing only the trusted ibc contract root, we can efficiently verify the IBC
key-value membership without needing to re-verifying the entire storage proof
against the state root for each IBC key-value.

### Negative

If we need to track other contract storage roots, we will need to maintain their
contract addresses as part of ClientState and track their storage roots in
ConsensusState.

### Neutral

This enables us to have a working IBC light client for Starknet without any
additional trust assumption.

## References

We have implemented block header verification and Starknet storage proof
verification as library.

- [starknet-block-verifier](https://github.com/informalsystems/ibc-starknet/tree/7fbbd89/light-client/starknet-block-verifier)
- [starknet-storage-verifier](https://github.com/informalsystems/ibc-starknet/tree/7fbbd89/light-client/starknet-storage-verifier)

These libraries are used in the IBC light client implementation.

- [verify_client_message](https://github.com/informalsystems/ibc-starknet/blob/7fbbd89/light-client/ibc-client-starknet/src/client_state/validation.rs#L46)
- [verify_membership_raw](https://github.com/informalsystems/ibc-starknet/blob/7fbbd89/light-client/ibc-client-starknet/src/client_state/validation.rs#L133)
- [verify_non_membership_raw](https://github.com/informalsystems/ibc-starknet/blob/7fbbd89/light-client/ibc-client-starknet/src/client_state/validation.rs#L178)
