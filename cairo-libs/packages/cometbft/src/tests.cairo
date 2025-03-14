use cometbft::ibc::Height;
use cometbft::light_client::Header;
use cometbft::types::{Commit, CommitSig, Consensus, Header as TmHeader, SignedHeader, ValidatorSet};
use protobuf::base64::decode as base64_decode;
use protobuf::types::message::ProtoCodecImpl;
use protobuf::types::wkt::Any;

#[test]
fn test_height_decode() {
    let base64 = "EEA=";
    let bytes = base64_decode(@base64);
    let height = ProtoCodecImpl::decode::<Height>(@bytes).unwrap();
    assert_eq!(
        height, Height { revision_number: 0, revision_height: 64 }, "height decode mismatch",
    );
    let bytes2 = ProtoCodecImpl::encode(@height);
    assert_eq!(bytes, bytes2, "height encode/decode mismatch");
}

#[test]
fn test_consensus_version() {
    let base64 = "CAs=";
    let bytes = base64_decode(@base64);
    let consensus = ProtoCodecImpl::decode::<Consensus>(@bytes).unwrap();
    assert_eq!(consensus, Consensus { block: 11, app: 0 }, "consensus decode mismatch");
    let bytes2 = ProtoCodecImpl::encode(@consensus);
    assert_eq!(bytes, bytes2, "consensus encode/decode mismatch");
}

#[test]
fn test_tm_header() {
    let base64 =
        "CgIICxIGbW9jay0wGEAiBgixm8OdBkIg5NIUfhxZlNr5WOr6hBNwbxx14aKBOizQ0yh2ol2bz5hKIOTSFH4cWZTa+Vjq+oQTcG8cdeGigTos0NModqJdm8+YUiDk0hR+HFmU2vlY6vqEE3BvHHXhooE6LNDTKHaiXZvPmFog1gQRGwpBP8v4ie4b2RKFCALfmmn1NjNTnPy2Mz7+WWtyFKbntoEN+BIFgPKoFxDiKPRU+ZyX";
    let bytes = base64_decode(@base64);
    let tm_header = ProtoCodecImpl::decode::<TmHeader>(@bytes).unwrap();
    let bytes2 = ProtoCodecImpl::encode(@tm_header);
    assert_eq!(bytes, bytes2, "tm_header encode/decode mismatch");
}

#[test]
fn test_commit_sig() {
    let base64 =
        "CAISFKbntoEN+BIFgPKoFxDiKPRU+ZyXGgYIsZvDnQYiQNAjIEO9oAEBbbn8ZYS6rK3/gx2dEeOcmNjdWJLu3LXbWVQG13EYpYU67kPMpYp7cWX/tMClKFpFKE5dDvlNEQk=";
    let bytes = base64_decode(@base64);
    let commit_sig = ProtoCodecImpl::decode::<CommitSig>(@bytes).unwrap();
    let bytes2 = ProtoCodecImpl::encode(@commit_sig);
    assert_eq!(bytes, bytes2, "commit_sig encode/decode mismatch");
}

#[test]
fn test_commit() {
    let base64 =
        "CEAQARpICiDBuFLkMsz2DNrMKJJaxK3uCD249XtqJJmRUHmDxS6qthIkCAESIMG4UuQyzPYM2swoklrEre4IPbj1e2okmZFQeYPFLqq2ImIIAhIUpue2gQ34EgWA8qgXEOIo9FT5nJcaBgixm8OdBiJA0CMgQ72gAQFtufxlhLqsrf+DHZ0R45yY2N1Yku7ctdtZVAbXcRilhTruQ8ylintxZf+0wKUoWkUoTl0O+U0RCSJiCAISFMeDImNgBHb9b/TFywqGCA0OX0iyGgYIsZvDnQYiQCpr0kBVoQAovJ/DHorjoM74rmdQiERWXuHUV9OSvEAyG48LrZR2rm2dBNU9wndYpLeenYKHHZEiR1Idq2pAKwM=";
    let bytes = base64_decode(@base64);
    let commit = ProtoCodecImpl::decode::<Commit>(@bytes).unwrap();
    let bytes2 = ProtoCodecImpl::encode(@commit);
    assert_eq!(bytes, bytes2, "commit encode/decode mismatch");
}


#[test]
fn test_validator_set() {
    let base64 =
        "CjwKFKbntoEN+BIFgPKoFxDiKPRU+ZyXEiIKIFDEpYca0zefKHnRLO91DRIRYzKDqcNzAjjm3fCE20yKGDIKPAoUx4MiY2AEdv1v9MXLCoYIDQ5fSLISIgog6+gLfK3qJ3rAX7hccWT+FevWhzxKdLMpakYqECb9mw8YMhhk";
    let bytes = base64_decode(@base64);
    let validator_set = ProtoCodecImpl::decode::<ValidatorSet>(@bytes).unwrap();
    let bytes2 = ProtoCodecImpl::encode(@validator_set);
    assert_eq!(bytes, bytes2, "validator_set encode/decode mismatch");
}

#[test]
fn test_signed_header() {
    let base64 =
        "CrQBCgIICxIGbW9jay0wGEAiBgixm8OdBkIg5NIUfhxZlNr5WOr6hBNwbxx14aKBOizQ0yh2ol2bz5hKIOTSFH4cWZTa+Vjq+oQTcG8cdeGigTos0NModqJdm8+YUiDk0hR+HFmU2vlY6vqEE3BvHHXhooE6LNDTKHaiXZvPmFog1gQRGwpBP8v4ie4b2RKFCALfmmn1NjNTnPy2Mz7+WWtyFKbntoEN+BIFgPKoFxDiKPRU+ZyXEpYCCEAQARpICiDBuFLkMsz2DNrMKJJaxK3uCD249XtqJJmRUHmDxS6qthIkCAESIMG4UuQyzPYM2swoklrEre4IPbj1e2okmZFQeYPFLqq2ImIIAhIUpue2gQ34EgWA8qgXEOIo9FT5nJcaBgixm8OdBiJA0CMgQ72gAQFtufxlhLqsrf+DHZ0R45yY2N1Yku7ctdtZVAbXcRilhTruQ8ylintxZf+0wKUoWkUoTl0O+U0RCSJiCAISFMeDImNgBHb9b/TFywqGCA0OX0iyGgYIsZvDnQYiQCpr0kBVoQAovJ/DHorjoM74rmdQiERWXuHUV9OSvEAyG48LrZR2rm2dBNU9wndYpLeenYKHHZEiR1Idq2pAKwM=";
    let bytes = base64_decode(@base64);
    let signed_header = ProtoCodecImpl::decode::<SignedHeader>(@bytes).unwrap();
    let bytes2 = ProtoCodecImpl::encode(@signed_header);
    assert_eq!(bytes, bytes2, "signed_header encode/decode mismatch");
}

#[test]
fn test_tm_lc_header_decode() {
    let base64 =
        "CtADCrQBCgIICxIGbW9jay0wGEAiBgixm8OdBkIg5NIUfhxZlNr5WOr6hBNwbxx14aKBOizQ0yh2ol2bz5hKIOTSFH4cWZTa+Vjq+oQTcG8cdeGigTos0NModqJdm8+YUiDk0hR+HFmU2vlY6vqEE3BvHHXhooE6LNDTKHaiXZvPmFog1gQRGwpBP8v4ie4b2RKFCALfmmn1NjNTnPy2Mz7+WWtyFKbntoEN+BIFgPKoFxDiKPRU+ZyXEpYCCEAQARpICiDBuFLkMsz2DNrMKJJaxK3uCD249XtqJJmRUHmDxS6qthIkCAESIMG4UuQyzPYM2swoklrEre4IPbj1e2okmZFQeYPFLqq2ImIIAhIUpue2gQ34EgWA8qgXEOIo9FT5nJcaBgixm8OdBiJA0CMgQ72gAQFtufxlhLqsrf+DHZ0R45yY2N1Yku7ctdtZVAbXcRilhTruQ8ylintxZf+0wKUoWkUoTl0O+U0RCSJiCAISFMeDImNgBHb9b/TFywqGCA0OX0iyGgYIsZvDnQYiQCpr0kBVoQAovJ/DHorjoM74rmdQiERWXuHUV9OSvEAyG48LrZR2rm2dBNU9wndYpLeenYKHHZEiR1Idq2pAKwMSfgo8ChSm57aBDfgSBYDyqBcQ4ij0VPmclxIiCiBQxKWHGtM3nyh50SzvdQ0SEWMyg6nDcwI45t3whNtMihgyCjwKFMeDImNgBHb9b/TFywqGCA0OX0iyEiIKIOvoC3yt6id6wF+4XHFk/hXr1oc8SnSzKWpGKhAm/ZsPGDIYZBoCEEAifgo8ChSm57aBDfgSBYDyqBcQ4ij0VPmclxIiCiBQxKWHGtM3nyh50SzvdQ0SEWMyg6nDcwI45t3whNtMihgyCjwKFMeDImNgBHb9b/TFywqGCA0OX0iyEiIKIOvoC3yt6id6wF+4XHFk/hXr1oc8SnSzKWpGKhAm/ZsPGDIYZA==";
    let bytes = base64_decode(@base64);
    let header = ProtoCodecImpl::decode::<Header>(@bytes).unwrap();
    let bytes2 = ProtoCodecImpl::encode(@header);
    assert_eq!(bytes, bytes2, "header encode/decode mismatch");
    let any: Any = header.clone().into();
    let header2: Header = any.try_into().unwrap();
    assert_eq!(header2, header, "header any encode/decode mismatch");
}

