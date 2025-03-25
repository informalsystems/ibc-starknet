use cometbft::light_client::Header as LcHeader;
use cometbft::types::{Header as TmHeader, UntrustedBlockState};
use cometbft::verifier::{header_matches_commit, verify_validator_sets};
use protobuf::types::message::ProtoCodecImpl;
use protobuf::{base64, hex};

#[test]
fn test_verify_validator_sets() {
    let data =
        "CtADCrQBCgIICxIGbW9jay0wGEAiBgixm8OdBkIg5NIUfhxZlNr5WOr6hBNwbxx14aKBOizQ0yh2ol2bz5hKIOTSFH4cWZTa+Vjq+oQTcG8cdeGigTos0NModqJdm8+YUiDk0hR+HFmU2vlY6vqEE3BvHHXhooE6LNDTKHaiXZvPmFog1gQRGwpBP8v4ie4b2RKFCALfmmn1NjNTnPy2Mz7+WWtyFKbntoEN+BIFgPKoFxDiKPRU+ZyXEpYCCEAQARpICiDBuFLkMsz2DNrMKJJaxK3uCD249XtqJJmRUHmDxS6qthIkCAESIMG4UuQyzPYM2swoklrEre4IPbj1e2okmZFQeYPFLqq2ImIIAhIUpue2gQ34EgWA8qgXEOIo9FT5nJcaBgixm8OdBiJA0CMgQ72gAQFtufxlhLqsrf+DHZ0R45yY2N1Yku7ctdtZVAbXcRilhTruQ8ylintxZf+0wKUoWkUoTl0O+U0RCSJiCAISFMeDImNgBHb9b/TFywqGCA0OX0iyGgYIsZvDnQYiQCpr0kBVoQAovJ/DHorjoM74rmdQiERWXuHUV9OSvEAyG48LrZR2rm2dBNU9wndYpLeenYKHHZEiR1Idq2pAKwMSfgo8ChSm57aBDfgSBYDyqBcQ4ij0VPmclxIiCiBQxKWHGtM3nyh50SzvdQ0SEWMyg6nDcwI45t3whNtMihgyCjwKFMeDImNgBHb9b/TFywqGCA0OX0iyEiIKIOvoC3yt6id6wF+4XHFk/hXr1oc8SnSzKWpGKhAm/ZsPGDIYZBoCEEAifgo8ChSm57aBDfgSBYDyqBcQ4ij0VPmclxIiCiBQxKWHGtM3nyh50SzvdQ0SEWMyg6nDcwI45t3whNtMihgyCjwKFMeDImNgBHb9b/TFywqGCA0OX0iyEiIKIOvoC3yt6id6wF+4XHFk/hXr1oc8SnSzKWpGKhAm/ZsPGDIYZA==";
    let bytes = base64::decode(@data);
    let header = ProtoCodecImpl::decode::<LcHeader>(@bytes).unwrap();

    let untrusted_block_state: UntrustedBlockState = header.into();
    verify_validator_sets(@untrusted_block_state);
}

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
