use ics23::{
    Proof, ProofSpec, RootBytes, ICS23Errors, ExistenceProofImpl, NonExistenceProofImpl,
    SliceU32IntoArrayU8
};

pub fn verify_membership(
    specs: Array<ProofSpec>,
    proofs: @Array<Proof>,
    root: RootBytes,
    keys: Array<ByteArray>,
    value: Array<u8>,
) {
    let proofs_len = proofs.len();
    assert(proofs_len > 0, ICS23Errors::MISSING_MERKLE_PROOF);
    assert(root != [0; 8], ICS23Errors::ZERO_MERKLE_ROOT);
    assert(value.len() > 0, ICS23Errors::MISSING_VALUE);
    assert(proofs_len == specs.len(), ICS23Errors::MISMATCHED_NUM_OF_PROOFS);
    assert(proofs_len == keys.len(), ICS23Errors::MISMATCHED_NUM_OF_PROOFS);
    let mut subroot = [0; 8];
    let mut subvalue: Array<u32> = ArrayTrait::new();
    let mut i = 0;
    while i < proofs_len {
        if let Proof::Exist(p) = proofs[i] {
            subroot = p.calculate_root();
            p.verify(specs[i], @subroot, keys[proofs_len - 1 - i], @value);
        } else {
            panic!("{}", ICS23Errors::INVALID_PROOF_TYPE);
        }
        subvalue = subroot.span().into();
        i += 1;
    };
    assert(root == subroot, ICS23Errors::INVALID_MERKLE_PROOF);
}

pub fn verify_non_membership(
    specs: Array<ProofSpec>, proofs: @Array<Proof>, root: RootBytes, keys: Array<ByteArray>
) {
    let proofs_len = proofs.len();
    assert(proofs_len > 0, ICS23Errors::MISSING_MERKLE_PROOF);
    assert(root == [0; 8], ICS23Errors::ZERO_MERKLE_ROOT);
    assert(proofs_len == specs.len(), ICS23Errors::MISMATCHED_NUM_OF_PROOFS);
    assert(proofs_len == keys.len(), ICS23Errors::MISMATCHED_NUM_OF_PROOFS);
    let mut subroot = [0; 8];
    let mut i = 0;
    while i < proofs_len {
        if let Proof::NonExist(p) = proofs[i] {
            subroot = p.calculate_root();
            p.verify(specs[i], @subroot, keys[proofs_len - 1 - i]);

            verify_membership(
                specs.clone(), proofs, root, keys.clone(), subroot.into()
            ) // TODO: add start_index
        } else {
            panic!("{}", ICS23Errors::INVALID_PROOF_TYPE);
        }
        i += 1;
    };
}
