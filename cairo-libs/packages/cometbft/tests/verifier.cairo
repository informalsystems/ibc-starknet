use cometbft::light_client::Header as LcHeader;
use cometbft::types::UntrustedBlockState;
use cometbft::verifier::verify_validator_sets;
use protobuf::base64::decode as base64_decode;
use protobuf::types::message::ProtoCodecImpl;

#[test]
fn test_verify_validator_sets() {
    let base64 =
        "CtADCrQBCgIICxIGbW9jay0wGEAiBgixm8OdBkIg5NIUfhxZlNr5WOr6hBNwbxx14aKBOizQ0yh2ol2bz5hKIOTSFH4cWZTa+Vjq+oQTcG8cdeGigTos0NModqJdm8+YUiDk0hR+HFmU2vlY6vqEE3BvHHXhooE6LNDTKHaiXZvPmFog1gQRGwpBP8v4ie4b2RKFCALfmmn1NjNTnPy2Mz7+WWtyFKbntoEN+BIFgPKoFxDiKPRU+ZyXEpYCCEAQARpICiDBuFLkMsz2DNrMKJJaxK3uCD249XtqJJmRUHmDxS6qthIkCAESIMG4UuQyzPYM2swoklrEre4IPbj1e2okmZFQeYPFLqq2ImIIAhIUpue2gQ34EgWA8qgXEOIo9FT5nJcaBgixm8OdBiJA0CMgQ72gAQFtufxlhLqsrf+DHZ0R45yY2N1Yku7ctdtZVAbXcRilhTruQ8ylintxZf+0wKUoWkUoTl0O+U0RCSJiCAISFMeDImNgBHb9b/TFywqGCA0OX0iyGgYIsZvDnQYiQCpr0kBVoQAovJ/DHorjoM74rmdQiERWXuHUV9OSvEAyG48LrZR2rm2dBNU9wndYpLeenYKHHZEiR1Idq2pAKwMSfgo8ChSm57aBDfgSBYDyqBcQ4ij0VPmclxIiCiBQxKWHGtM3nyh50SzvdQ0SEWMyg6nDcwI45t3whNtMihgyCjwKFMeDImNgBHb9b/TFywqGCA0OX0iyEiIKIOvoC3yt6id6wF+4XHFk/hXr1oc8SnSzKWpGKhAm/ZsPGDIYZBoCEEAifgo8ChSm57aBDfgSBYDyqBcQ4ij0VPmclxIiCiBQxKWHGtM3nyh50SzvdQ0SEWMyg6nDcwI45t3whNtMihgyCjwKFMeDImNgBHb9b/TFywqGCA0OX0iyEiIKIOvoC3yt6id6wF+4XHFk/hXr1oc8SnSzKWpGKhAm/ZsPGDIYZA==";
    let bytes = base64_decode(@base64);
    let header = ProtoCodecImpl::decode::<LcHeader>(@bytes).unwrap();
    let untrusted_block_state: UntrustedBlockState = header.into();
    verify_validator_sets(@untrusted_block_state);
}
