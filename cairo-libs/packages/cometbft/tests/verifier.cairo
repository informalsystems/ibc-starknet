use cometbft::light_client::Header as LcHeader;
use cometbft::types::{Header as TmHeader, UntrustedBlockState};
use cometbft::verifier::{header_matches_commit, verify_validator_sets};
use protobuf::types::message::ProtoCodecImpl;
use protobuf::{base64, hex};

// TODO(rano): generate a verify_update_header using ibc-rs. and test here.

#[test]
fn test_verify_commit_hash() {
    // https://github.com/informalsystems/tendermint-rs/blob/6cc391c80ae988615508bd87285571ba130b604c/tendermint/src/block/header.rs#L270-L281
    let header_data =
        "CgQICxABEgtkb2NrZXJjaGFpbhjIDCILCPK2/voFEOKbhmUqSAog07LMftr/h0M6Xbzc30B3pWqs3jYGA0JisM2xIPYutAsSJAgBEiA6tBHq/po7esATsCFJkOVlMRKjmQkonj6pIR8HuM1u7TIgRwcbhu/Ci+wXVDlnl181GRupvsnCrXfob2OxSVKNcaE6IOOwxEKY/BwUmvv0yJlvuSQnrkHkZJuTTKSVmRt4UrhVQiBeIFIOyAuEBEtkugxVscBtVDu9V5VcJ7ipmZ7FJr9wPEogXiBSDsgLhARLZLoMVbHAbVQ7vVeVXCe4qZmexSa/cDxSIASAkbx93Cg/d7+/kdc8RNpYw9+KnLyGdAXYt/ParaIvWggAAAAAAAAAAGIg47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFVqIOOwxEKY/BwUmvv0yJlvuSQnrkHkZJuTTKSVmRt4UrhVchTIZXow0gw7rUFGJKGpYzc91QDM0w==";
    let expected_hash_data = "F30A71F2409FB15AACAEDB6CC122DFA2525BEE9CAE521721B06BFDCA291B8D56";

    let header_bytes = base64::decode(@header_data);
    let expected_hash = hex::decode(@expected_hash_data);

    let expected_hash_array = {
        let mut ar = array![];
        let mut i = 0;

        while i < expected_hash.len() {
            ar.append(expected_hash[i]);
            i += 1;
        }

        ar
    };

    let header = ProtoCodecImpl::decode::<TmHeader>(@header_bytes).unwrap();
    header_matches_commit(@header, @expected_hash_array);
}
