use cometbft::light_client::Header as LcHeader;
use cometbft::types::{Header as TmHeader, Options, TrustedBlockState, UntrustedBlockState};
use cometbft::utils::TWO_THIRDS;
use cometbft::verifier::{header_matches_commit, verify_update_header};
use protobuf::types::message::ProtoCodecImpl;
use protobuf::types::wkt::{Duration, Timestamp};
use protobuf::{base64, hex};

fn header_fixture() -> (LcHeader, LcHeader) {
    // the base64 data is grabbed from e2e tests

    let header_31_data =
        "CtMECpcDCgIICxIRY29zbW9zLTE4ODM0OTA4MjEYHyIMCMOvqr8GEP+a6YIBKkgKIIdYpjU6YCGuoWfe72fR6miniKYU6U4ARt+hy3QKeNMJEiQIARIgMv+4MPJgglT4k4rJmf49tDr+yZ7xiDZOMppXjTYe+HcyIJFV9zhEOUM7Mrlah+f73qqAY3W/aIOhFlVG2JqeBdp6OiDjsMRCmPwcFJr79MiZb7kkJ65B5GSbk0yklZkbeFK4VUIgKghhc7iMawF0w1OS0HrVbz7AgQZ5LBdTPUZMkJLL7MFKICoIYXO4jGsBdMNTktB61W8+wIEGeSwXUz1GTJCSy+zBUiAEgJG8fdwoP3e/v5HXPETaWMPfipy8hnQF2Lfz2q2iL1ogtFEq6xk4U8jsLc1ZsnIwQ1SSVsIOuvi+DJ2YhgwAObRiIOOwxEKY/BwUmvv0yJlvuSQnrkHkZJuTTKSVmRt4UrhVaiDjsMRCmPwcFJr79MiZb7kkJ65B5GSbk0yklZkbeFK4VXIUnX5Uljlx7LVtbBKgIuc5AVYOYD8StgEIHxpICiBFbH6PTs+RKnWIisUjIm0r2FVcfb3kHyAfxTkXIeobexIkCAESIM/53utP9Hgc5dy4OmAk19VQEDTJAQPoNJb81DS33a09ImgIAhIUnX5Uljlx7LVtbBKgIuc5AVYOYD8aDAjEr6q/BhCR6o+uASJAUGBSHRTuKbzMGaW7L1FBB/pE1KThO94Sadysb/IUZW2vFy57wAVJ720hkAdZFNnFVDoZQHqzZOvV4La6xk0uCBKNAQpBChSdflSWOXHstW1sEqAi5zkBVg5gPxIiCiA57g6RpkaiCURNYrCSk8ORUAPEwW2GNEkGlL3jxdVtzhiAoJSljR0SQQoUnX5Uljlx7LVtbBKgIuc5AVYOYD8SIgogOe4OkaZGoglETWKwkpPDkVADxMFthjRJBpS948XVbc4YgKCUpY0dGICglKWNHRoICIWUj4IHEBMijQEKQQoUnX5Uljlx7LVtbBKgIuc5AVYOYD8SIgogOe4OkaZGoglETWKwkpPDkVADxMFthjRJBpS948XVbc4YgKCUpY0dEkEKFJ1+VJY5cey1bWwSoCLnOQFWDmA/EiIKIDnuDpGmRqIJRE1isJKTw5FQA8TBbYY0SQaUvePF1W3OGICglKWNHRiAoJSljR0=";
    let header_43_data =
        "CtMECpcDCgIICxIRY29zbW9zLTE4ODM0OTA4MjEYKyIMCNCvqr8GEI+fmoQBKkgKIM/ZJJBezJ1/N5oZ81wGNakLmvdXqJUTeo9BflXGB/CVEiQIARIgGavZsDudynigNSd0t9pB3gsLO4jBPiTAvg68tr9bSK8yILQDlUyLpWuELNkvXzc0wYO6HX7p8iBNs0LTSm9XUXUNOiDjsMRCmPwcFJr79MiZb7kkJ65B5GSbk0yklZkbeFK4VUIgKghhc7iMawF0w1OS0HrVbz7AgQZ5LBdTPUZMkJLL7MFKICoIYXO4jGsBdMNTktB61W8+wIEGeSwXUz1GTJCSy+zBUiAEgJG8fdwoP3e/v5HXPETaWMPfipy8hnQF2Lfz2q2iL1ogzm0HC2rcfn6E1osiy9/b1EghkZPbwy12IUhatZAb15JiIOOwxEKY/BwUmvv0yJlvuSQnrkHkZJuTTKSVmRt4UrhVaiDjsMRCmPwcFJr79MiZb7kkJ65B5GSbk0yklZkbeFK4VXIUnX5Uljlx7LVtbBKgIuc5AVYOYD8StgEIKxpICiC2aFSjURAzE+gYhms6RpW76keJyGqQVkoKVSv6IUzorBIkCAESINciCYx+c3vGBhxjY9LDgKvTBFjXlXfdKHreQ0c5J7HjImgIAhIUnX5Uljlx7LVtbBKgIuc5AVYOYD8aDAjRr6q/BhDDypWtASJAsBQH8E+My3FUmQTbx+j0g+0emNYXNV1wiirfrqJRJUnLhTYXwnLaQSBXuARAQYp9E+K8XT6jVtAjRQfZby8aCRKNAQpBChSdflSWOXHstW1sEqAi5zkBVg5gPxIiCiA57g6RpkaiCURNYrCSk8ORUAPEwW2GNEkGlL3jxdVtzhiAoJSljR0SQQoUnX5Uljlx7LVtbBKgIuc5AVYOYD8SIgogOe4OkaZGoglETWKwkpPDkVADxMFthjRJBpS948XVbc4YgKCUpY0dGICglKWNHRoICIWUj4IHEB8ijQEKQQoUnX5Uljlx7LVtbBKgIuc5AVYOYD8SIgogOe4OkaZGoglETWKwkpPDkVADxMFthjRJBpS948XVbc4YgKCUpY0dEkEKFJ1+VJY5cey1bWwSoCLnOQFWDmA/EiIKIDnuDpGmRqIJRE1isJKTw5FQA8TBbYY0SQaUvePF1W3OGICglKWNHRiAoJSljR0=";

    let header_31_bytes = base64::decode(@header_31_data);
    let header_43_bytes = base64::decode(@header_43_data);

    let header_31 = ProtoCodecImpl::decode::<LcHeader>(@header_31_bytes).unwrap();
    let header_43 = ProtoCodecImpl::decode::<LcHeader>(@header_43_bytes).unwrap();

    assert_eq!(@header_31.signed_header.header.height, @31);
    assert_eq!(@header_43.signed_header.header.height, @43);

    assert_eq!(@header_31.signed_header.header.chain_id, @header_43.signed_header.header.chain_id);
    assert_lt!(@header_31.signed_header.header.time, @header_43.signed_header.header.time);

    assert_eq!(
        @header_31.signed_header.header.next_validators_hash,
        @header_43.signed_header.header.validators_hash,
    );

    (header_31, header_43)
}

#[test]
fn test_verify_update_header() {
    let (header_a, header_b) = header_fixture();

    let trusted_block_state = TrustedBlockState {
        chain_id: header_a.signed_header.header.chain_id.clone(),
        header_time: header_a.signed_header.header.time.clone(),
        height: header_a.signed_header.header.height,
        next_validators: header_b.validator_set.clone(), // full validator_set is in future header
        next_validators_hash: header_a.signed_header.header.next_validators_hash.clone(),
    };

    let untrusted_block_state = UntrustedBlockState {
        signed_header: header_b.signed_header.clone(), validators: header_b.validator_set,
    };

    let trusting_period = Duration { seconds: 1209600, nanos: 0 };

    let clock_drift = Duration { seconds: 3, nanos: 0 };

    let now = Timestamp {
        // header is submitted 30 seconds later
        seconds: header_b.signed_header.header.time.seconds + 30,
        nanos: header_b.signed_header.header.time.nanos,
    };

    let options = Options { trust_threshold: TWO_THIRDS, trusting_period, clock_drift };

    verify_update_header(untrusted_block_state, trusted_block_state, options, now);
}

#[test]
#[should_panic(expected: 'ICS07: invalid commit hash')]
fn test_verify_update_header_forged_header() {
    let (header_a, header_b) = header_fixture();

    let trusted_block_state = TrustedBlockState {
        chain_id: header_a.signed_header.header.chain_id.clone(),
        header_time: header_a.signed_header.header.time.clone(),
        height: header_a.signed_header.header.height,
        next_validators: header_b.validator_set.clone(), // full validator_set is in future header
        next_validators_hash: header_a.signed_header.header.next_validators_hash.clone(),
    };

    let mut untrusted_block_state = UntrustedBlockState {
        signed_header: header_b.signed_header.clone(), validators: header_b.validator_set,
    };

    let trusting_period = Duration { seconds: 1209600, nanos: 0 };

    let clock_drift = Duration { seconds: 3, nanos: 0 };

    let now = Timestamp {
        // header is submitted 30 seconds later
        seconds: header_b.signed_header.header.time.seconds + 30,
        nanos: header_b.signed_header.header.time.nanos,
    };

    let options = Options { trust_threshold: TWO_THIRDS, trusting_period, clock_drift };

    // forged header
    untrusted_block_state.signed_header.header.next_validators_hash = array![0x1, 0x2];

    verify_update_header(untrusted_block_state, trusted_block_state, options, now);
}

#[test]
#[should_panic(expected: 'ICS07: invalid sig count')]
fn test_verify_update_header_empty_signatures() {
    let (header_a, header_b) = header_fixture();

    let trusted_block_state = TrustedBlockState {
        chain_id: header_a.signed_header.header.chain_id.clone(),
        header_time: header_a.signed_header.header.time.clone(),
        height: header_a.signed_header.header.height,
        next_validators: header_b.validator_set.clone(), // full validator_set is in future header
        next_validators_hash: header_a.signed_header.header.next_validators_hash.clone(),
    };

    let mut untrusted_block_state = UntrustedBlockState {
        signed_header: header_b.signed_header.clone(), validators: header_b.validator_set,
    };

    let trusting_period = Duration { seconds: 1209600, nanos: 0 };

    let clock_drift = Duration { seconds: 3, nanos: 0 };

    let now = Timestamp {
        // header is submitted 30 seconds later
        seconds: header_b.signed_header.header.time.seconds + 30,
        nanos: header_b.signed_header.header.time.nanos,
    };

    let options = Options { trust_threshold: TWO_THIRDS, trusting_period, clock_drift };

    // empty signatures
    untrusted_block_state.signed_header.commit.signatures = array![];

    verify_update_header(untrusted_block_state, trusted_block_state, options, now);
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
