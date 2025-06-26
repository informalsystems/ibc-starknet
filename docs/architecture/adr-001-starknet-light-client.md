# ADR 001: Starknet Light Client

## Changelog

- 2025-06-25: Initial version

## Context

We are building an IBC between Cosmos-SDK chains and Starknet. For this, we need
a light client of Starknet that can verify the validity of Starknet block
headers and the vector commitments of the committed IBC values.

## Decision

After discussing different approaches, we have decided to implement a light
client by verifying the signature of the Starknet centralized sequencer for each
block header.

The signature verification is done by:

- [hashing the block header](https://github.com/informalsystems/ibc-starknet/blob/3e6f71ed02f68e343b03b79d686ac6f10e7aef7a/light-client/starknet-block-verifier/src/types.rs#L128)
  following the Starknet sequencer implementation.
- and then verifying the
  [signature of the sequencer for the hashed value](https://github.com/informalsystems/ibc-starknet/blob/3e6f71ed02f68e343b03b79d686ac6f10e7aef7a/light-client/starknet-block-verifier/src/types.rs#L158-L163).

The full implementation can be found at
[starknet-block-verifier](https://github.com/informalsystems/ibc-starknet/blob/3e6f71ed02f68e343b03b79d686ac6f10e7aef7a/light-client/starknet-block-verifier/src/types.rs#L151)
crate.

Once we have verified the block header, we can trust the state root in it.

This trusted state root is then used to verify the vector commitments of the
committed IBC values following the block structure and the Merkle tree of
Starknet key-value storage.

Now there are three parts to the vector commitment verification given a trusted
state root:

- the committed IBC values are stored in the ibc-core contract storage, which
  has its own trie root.
- this Merkle root is then stored in a global trie storage mapping each deployed
  contract to its storage root hash.
- the state root of the block header is then computed using the global
  `contract_trie_root` and `class_trie_root`.

Note that, for a different IBC key-value, only the first step is different. The
following two steps are always the same a trusted state root. So, to avoid
unnecessary repeated computed, we decided to include the last two steps in the
client update and store only the trie root of the ibc-core contract storage.

### Client Types

```rust
pub struct ClientState {
    pub chain_id: Felt,
    pub max_clock_drift: u64,
    pub latest_height: u64,

    pub ibc_core_contract_address: Felt,
}

pub struct ConsensusState {
    pub timestamp: u64,
    pub ibc_core_trie_root: Felt,
}

pub struct Header {
    pub starknet_header: BlockHeader,
    
    pub contract_trie_root: Felt,
    pub class_trie_root: Felt,

    pub contract_trie_root_proof: ContractStorageProof,
    pub ibc_core_trie_root: Felt,
}
```

### Verifying the Block Header

```rust
fn verify_header(
    header: Header, // untrusted; given by the relayer
    contract_trie_root: Felt, // untrusted; given by the relayer
    class_trie_root: Felt, // untrusted; given by the relayer
    ibc_core_contract_address: Felt, // trusted; given by the ibc module
    ibc_core_contract_root: Felt, // untrusted; given by the relayer
    proof: StarknetStorageProof, // untrusted; given by the relayer
) -> Felt {
    assert!(header.verify_signature());

    // header is now trusted

    let expected_state_root = header.state_root;

    let actual_state_root = compute_global_state_root(contract_trie_root, class_trie_root);

    assert_eq!(
        expected_state_root, actual_state_root,
        "State root mismatch"
    );

    // contract_trie_root is now trusted

    assert!(proof.verify(contract_trie_root, ibc_core_contract_address, ibc_core_contract_root));

    // ibc_core_contract_root is now trusted
}
```

### Verifying the IBC Key-Value Membership

```rust
fn verify_membership(
    ibc_core_contract_trie_root: Felt, // trusted; given by the ibc module
    key: Felt, // trusted; given by the ibc module
    // if value is None, we are checking for non-membership
    value: Option<Felt>, // untrusted; given by the relayer
    proof: StarknetStorageProof, // untrusted; given by the relayer
) -> bool {
    assert!(proof.verify(contract_root, key, value));

    // value is now trusted
}
```

## Status

Proposed

## Consequences

### Positive

This enables us to have a working IBC light client for Starknet. By storing only
the trusted ibc-core contract trie root, we can efficiently verify the IBC
key-value membership without needing to re-verify the entire vector commitment
for each IBC key-value.

### Negative

N/A

### Neutral

N/A

## References

We have already implemented block header verification and Starknet storage proof
verification.

- [starknet-block-verifier](https://github.com/informalsystems/ibc-starknet/tree/main/light-client/starknet-block-verifier)
- [starknet-storage-verifier](https://github.com/informalsystems/ibc-starknet/tree/main/light-client/starknet-storage-verifier)

But the implementations may need some adjustments to fit this ADR.

- We verify the Starknet storage proofs only against the `contract_trie_root`.
  We need to add the step of verifying the state root recomputed from the global
  `contract_trie_root` and `class_trie_root`.
  - The `compute_global_state_root` function can be implemented following
    reference implementation in
    [starkware-libs/sequencer](https://github.com/starkware-libs/sequencer/blob/12a461e6604f29538b24d68903cbd33f44fdf2fe/crates/apollo_starknet_os_program/src/cairo/starkware/starknet/core/os/state/commitment.cairo#L34).
- We verify the Starknet storage proofs directly against the
  `contract_trie_root`. But we want to move the verification of ibc-core
  contract trie-root in the client update step. So, we need to split the two
  ([1](https://github.com/informalsystems/ibc-starknet/blob/3e6f71ed02f68e343b03b79d686ac6f10e7aef7a/light-client/starknet-storage-verifier/src/storage/verifier.rs#L199),
  [2](https://github.com/informalsystems/ibc-starknet/blob/3e6f71ed02f68e343b03b79d686ac6f10e7aef7a/light-client/starknet-storage-verifier/src/storage/verifier.rs#L207))
  merkle tree verification steps into two separate functions.
