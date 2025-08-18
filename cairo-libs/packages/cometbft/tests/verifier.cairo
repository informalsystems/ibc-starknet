use cometbft::light_client::Header as LcHeader;
use cometbft::types::{Header as TmHeader, Options, TrustedBlockState, UntrustedBlockState};
use cometbft::utils::TWO_THIRDS;
use cometbft::verifier::{header_matches_commit, verify_update_header};
use ibc_utils::bytes::ByteArrayIntoArrayU8;
use ibc_utils::{base64, hex};
use protobuf::types::message::ProtoCodecImpl;
use protobuf::types::wkt::{Duration, Timestamp};

fn header_fixture() -> (LcHeader, LcHeader) {
    // the base64 data is grabbed from e2e tests

    let header_31_data =
        "CtMECpcDCgIICxIRY29zbW9zLTE4ODM0OTA4MjEYHyIMCMOvqr8GEP+a6YIBKkgKIIdYpjU6YCGuoWfe72fR6miniKYU6U4ARt+hy3QKeNMJEiQIARIgMv+4MPJgglT4k4rJmf49tDr+yZ7xiDZOMppXjTYe+HcyIJFV9zhEOUM7Mrlah+f73qqAY3W/aIOhFlVG2JqeBdp6OiDjsMRCmPwcFJr79MiZb7kkJ65B5GSbk0yklZkbeFK4VUIgKghhc7iMawF0w1OS0HrVbz7AgQZ5LBdTPUZMkJLL7MFKICoIYXO4jGsBdMNTktB61W8+wIEGeSwXUz1GTJCSy+zBUiAEgJG8fdwoP3e/v5HXPETaWMPfipy8hnQF2Lfz2q2iL1ogtFEq6xk4U8jsLc1ZsnIwQ1SSVsIOuvi+DJ2YhgwAObRiIOOwxEKY/BwUmvv0yJlvuSQnrkHkZJuTTKSVmRt4UrhVaiDjsMRCmPwcFJr79MiZb7kkJ65B5GSbk0yklZkbeFK4VXIUnX5Uljlx7LVtbBKgIuc5AVYOYD8StgEIHxpICiBFbH6PTs+RKnWIisUjIm0r2FVcfb3kHyAfxTkXIeobexIkCAESIM/53utP9Hgc5dy4OmAk19VQEDTJAQPoNJb81DS33a09ImgIAhIUnX5Uljlx7LVtbBKgIuc5AVYOYD8aDAjEr6q/BhCR6o+uASJAUGBSHRTuKbzMGaW7L1FBB/pE1KThO94Sadysb/IUZW2vFy57wAVJ720hkAdZFNnFVDoZQHqzZOvV4La6xk0uCBKNAQpBChSdflSWOXHstW1sEqAi5zkBVg5gPxIiCiA57g6RpkaiCURNYrCSk8ORUAPEwW2GNEkGlL3jxdVtzhiAoJSljR0SQQoUnX5Uljlx7LVtbBKgIuc5AVYOYD8SIgogOe4OkaZGoglETWKwkpPDkVADxMFthjRJBpS948XVbc4YgKCUpY0dGICglKWNHRoICIWUj4IHEBMijQEKQQoUnX5Uljlx7LVtbBKgIuc5AVYOYD8SIgogOe4OkaZGoglETWKwkpPDkVADxMFthjRJBpS948XVbc4YgKCUpY0dEkEKFJ1+VJY5cey1bWwSoCLnOQFWDmA/EiIKIDnuDpGmRqIJRE1isJKTw5FQA8TBbYY0SQaUvePF1W3OGICglKWNHRiAoJSljR0=";
    let header_43_data =
        "CtMECpcDCgIICxIRY29zbW9zLTE4ODM0OTA4MjEYKyIMCNCvqr8GEI+fmoQBKkgKIM/ZJJBezJ1/N5oZ81wGNakLmvdXqJUTeo9BflXGB/CVEiQIARIgGavZsDudynigNSd0t9pB3gsLO4jBPiTAvg68tr9bSK8yILQDlUyLpWuELNkvXzc0wYO6HX7p8iBNs0LTSm9XUXUNOiDjsMRCmPwcFJr79MiZb7kkJ65B5GSbk0yklZkbeFK4VUIgKghhc7iMawF0w1OS0HrVbz7AgQZ5LBdTPUZMkJLL7MFKICoIYXO4jGsBdMNTktB61W8+wIEGeSwXUz1GTJCSy+zBUiAEgJG8fdwoP3e/v5HXPETaWMPfipy8hnQF2Lfz2q2iL1ogzm0HC2rcfn6E1osiy9/b1EghkZPbwy12IUhatZAb15JiIOOwxEKY/BwUmvv0yJlvuSQnrkHkZJuTTKSVmRt4UrhVaiDjsMRCmPwcFJr79MiZb7kkJ65B5GSbk0yklZkbeFK4VXIUnX5Uljlx7LVtbBKgIuc5AVYOYD8StgEIKxpICiC2aFSjURAzE+gYhms6RpW76keJyGqQVkoKVSv6IUzorBIkCAESINciCYx+c3vGBhxjY9LDgKvTBFjXlXfdKHreQ0c5J7HjImgIAhIUnX5Uljlx7LVtbBKgIuc5AVYOYD8aDAjRr6q/BhDDypWtASJAsBQH8E+My3FUmQTbx+j0g+0emNYXNV1wiirfrqJRJUnLhTYXwnLaQSBXuARAQYp9E+K8XT6jVtAjRQfZby8aCRKNAQpBChSdflSWOXHstW1sEqAi5zkBVg5gPxIiCiA57g6RpkaiCURNYrCSk8ORUAPEwW2GNEkGlL3jxdVtzhiAoJSljR0SQQoUnX5Uljlx7LVtbBKgIuc5AVYOYD8SIgogOe4OkaZGoglETWKwkpPDkVADxMFthjRJBpS948XVbc4YgKCUpY0dGICglKWNHRoICIWUj4IHEB8ijQEKQQoUnX5Uljlx7LVtbBKgIuc5AVYOYD8SIgogOe4OkaZGoglETWKwkpPDkVADxMFthjRJBpS948XVbc4YgKCUpY0dEkEKFJ1+VJY5cey1bWwSoCLnOQFWDmA/EiIKIDnuDpGmRqIJRE1isJKTw5FQA8TBbYY0SQaUvePF1W3OGICglKWNHRiAoJSljR0=";

    let header_31_bytes = base64::decode_byte_array(header_31_data);
    let header_43_bytes = base64::decode_byte_array(header_43_data);

    let header_31 = ProtoCodecImpl::decode::<LcHeader>(header_31_bytes.span()).unwrap();
    let header_43 = ProtoCodecImpl::decode::<LcHeader>(header_43_bytes.span()).unwrap();

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
fn test_header_serde() {
    let (header_a, _) = header_fixture();

    let mut header_a_serialized = array![];

    Serde::serialize(@header_a, ref header_a_serialized);

    let mut header_a_serialized = header_a_serialized.span();

    let header_a_deserialized = Serde::<LcHeader>::deserialize(ref header_a_serialized).unwrap();

    assert_eq!(@header_a, @header_a_deserialized);
}

#[test]
fn test_verify_update_header() {
    let (header_a, header_b) = header_fixture();

    let trusting_period = Duration { seconds: 1209600, nanos: 0 };

    let clock_drift = Duration { seconds: 3, nanos: 0 };

    let options = Options { trust_threshold: TWO_THIRDS, trusting_period, clock_drift };

    let now = Timestamp {
        // header is submitted 30 seconds later
        seconds: header_b.signed_header.header.time.seconds + 30,
        nanos: header_b.signed_header.header.time.nanos,
    };

    let trusted_block_state = TrustedBlockState {
        chain_id: header_a.signed_header.header.chain_id,
        header_time: header_a.signed_header.header.time,
        height: header_a.signed_header.header.height,
        next_validators: header_b.validator_set.clone(), // full validator_set is in future header
        next_validators_hash: header_a.signed_header.header.next_validators_hash,
    };

    let untrusted_block_state = UntrustedBlockState {
        signed_header: header_b.signed_header,
        validators: header_b.validator_set,
        next_validators: header_b.trusted_validator_set,
    };

    let mut signature_hints = array![
        array![
            0x14, 0x8c47b3a76233d918b5b26650, 0x7046c077146ceef5bd5a9cd0, 0x1c0725137a073f62, 0x0,
            0xbafb4949da9388fbb985ab46, 0x98e154085ca0ec2c3155bde3, 0x6a0fc67575c4f06f, 0x0,
            0x819af043bc027f62320d1d38299941b, 0x12cbd2baa8287b8f38e80f9b85de68b06,
            0xc311be8d5eff352810d7b49c, 0xf8911ff5188ca79694472a8, 0x3d51246cb697b962, 0x0,
            0x7e5fe2178d696fb5b39c879d, 0x85a72c498f32f122061d4ff3, 0x2c72d8a40908d78a, 0x0,
            0x1e1bd09b9b9302ca7491500c1e704208, 0x2cd09f0ce2e90fe730c1f60986ee2847,
            0x8966bb37961bf3661b4a764489b1cba, 0x4c672ee952b599423ca4b1487f4c1865,
            0x3b00b89a97805b3479ecb70ab387d27f, 0x59ad8bb7321f66443ec98ea59b1e6da7,
        ],
    ];

    verify_update_header(
        untrusted_block_state, trusted_block_state, options, now, array![].span(), signature_hints,
    );
}

#[test]
#[should_panic(expected: 'ICS07: invalid val set hash')]
fn test_verify_update_header_forged_header() {
    let (header_a, header_b) = header_fixture();

    let trusting_period = Duration { seconds: 1209600, nanos: 0 };

    let clock_drift = Duration { seconds: 3, nanos: 0 };

    let options = Options { trust_threshold: TWO_THIRDS, trusting_period, clock_drift };

    let now = Timestamp {
        // header is submitted 30 seconds later
        seconds: header_b.signed_header.header.time.seconds + 30,
        nanos: header_b.signed_header.header.time.nanos,
    };

    let trusted_block_state = TrustedBlockState {
        chain_id: header_a.signed_header.header.chain_id,
        header_time: header_a.signed_header.header.time,
        height: header_a.signed_header.header.height,
        next_validators: header_b.validator_set.clone(), // full validator_set is in future header
        next_validators_hash: header_a.signed_header.header.next_validators_hash,
    };

    let mut untrusted_block_state = UntrustedBlockState {
        signed_header: header_b.signed_header,
        validators: header_b.validator_set,
        next_validators: header_b.trusted_validator_set,
    };

    // forged header
    untrusted_block_state.signed_header.header.next_validators_hash = array![0x1, 0x2];

    let mut signature_hints = array![array![]];

    verify_update_header(
        untrusted_block_state, trusted_block_state, options, now, array![].span(), signature_hints,
    );
}

#[test]
#[should_panic(expected: 'ICS07: invalid sig count')]
fn test_verify_update_header_empty_signatures() {
    let (header_a, header_b) = header_fixture();

    let trusting_period = Duration { seconds: 1209600, nanos: 0 };

    let clock_drift = Duration { seconds: 3, nanos: 0 };

    let options = Options { trust_threshold: TWO_THIRDS, trusting_period, clock_drift };

    let now = Timestamp {
        // header is submitted 30 seconds later
        seconds: header_b.signed_header.header.time.seconds + 30,
        nanos: header_b.signed_header.header.time.nanos,
    };

    let trusted_block_state = TrustedBlockState {
        chain_id: header_a.signed_header.header.chain_id,
        header_time: header_a.signed_header.header.time,
        height: header_a.signed_header.header.height,
        next_validators: header_b.validator_set.clone(), // full validator_set is in future header
        next_validators_hash: header_a.signed_header.header.next_validators_hash,
    };

    let mut untrusted_block_state = UntrustedBlockState {
        signed_header: header_b.signed_header,
        validators: header_b.validator_set,
        next_validators: header_b.trusted_validator_set,
    };

    // empty signatures
    untrusted_block_state.signed_header.commit.signatures = array![];

    let mut signature_hints = array![];

    verify_update_header(
        untrusted_block_state, trusted_block_state, options, now, array![].span(), signature_hints,
    );
}

#[test]
fn test_verify_commit_hash() {
    // https://github.com/informalsystems/tendermint-rs/blob/6cc391c80ae988615508bd87285571ba130b604c/tendermint/src/block/header.rs#L270-L281
    let header_data =
        "CgQICxABEgtkb2NrZXJjaGFpbhjIDCILCPK2/voFEOKbhmUqSAog07LMftr/h0M6Xbzc30B3pWqs3jYGA0JisM2xIPYutAsSJAgBEiA6tBHq/po7esATsCFJkOVlMRKjmQkonj6pIR8HuM1u7TIgRwcbhu/Ci+wXVDlnl181GRupvsnCrXfob2OxSVKNcaE6IOOwxEKY/BwUmvv0yJlvuSQnrkHkZJuTTKSVmRt4UrhVQiBeIFIOyAuEBEtkugxVscBtVDu9V5VcJ7ipmZ7FJr9wPEogXiBSDsgLhARLZLoMVbHAbVQ7vVeVXCe4qZmexSa/cDxSIASAkbx93Cg/d7+/kdc8RNpYw9+KnLyGdAXYt/ParaIvWggAAAAAAAAAAGIg47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFVqIOOwxEKY/BwUmvv0yJlvuSQnrkHkZJuTTKSVmRt4UrhVchTIZXow0gw7rUFGJKGpYzc91QDM0w==";
    let expected_hash_data = "F30A71F2409FB15AACAEDB6CC122DFA2525BEE9CAE521721B06BFDCA291B8D56";

    let header_bytes = base64::decode_byte_array(header_data);
    let expected_hash = hex::decode_byte_array(expected_hash_data);

    let header = ProtoCodecImpl::decode::<TmHeader>(header_bytes.span()).unwrap();
    header_matches_commit(@header, expected_hash.span());
}

#[test]
fn test_verify_update_header_2() {
    let (header_a, header_b) = {
        let header_31_data =
            "CtQECpgDCgIICxISY2hhaW4tMS0zOTY1MTkyNTI0GEUiDAjBj/S/BhDRjtP5AipICiBV2gWqX6CI8jJVAHN1rTgL8wEsBguLKZYqpmQ+ePw6zxIkCAESIGndiTR+3rTntsuzRvQQR8/R2AGMVDaZXStS0BvZdu99MiD++0w9b/uWxtMfeDgUMdpAEAW1m6qSq/ZAOPffP/PVYTog+IAv/1bVmWQydSI5FkTXZ21n9kboygWuJ+GxjUIPHftCINcW8oxHV55BXwPjDwGa1MD56V3MRivoxwHwmFs+txiFSiDXFvKMR1eeQV8D4w8BmtTA+eldzEYr6McB8JhbPrcYhVIgBICRvH3cKD93v7+R1zxE2ljD34qcvIZ0Bdi389qtoi9aIKdlsaLtsHbZCc6JFZxRu9ivZMfUIb3G2y6TjcDuV3spYiDjsMRCmPwcFJr79MiZb7kkJ65B5GSbk0yklZkbeFK4VWog47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFVyFDSGI1jd7XjyZhOZj/vLrGKRTVR8ErYBCEUaSAog6/hVz9iZlDCSHdyOh8AkrfphKXM0mQjSh2Xu6YBDzacSJAgBEiBkWzJJVKt5MlUREPXcsxhLGWgelN0hNfqiherzFTwWSSJoCAISFDSGI1jd7XjyZhOZj/vLrGKRTVR8GgwIwo/0vwYQ3KjL/wIiQHbxSVVT+bnFgaVIeIa1mKzSVuoXYFsDEmOzZlzazzacKiOOrY1zkZnZNf82Az1w5VbcDN9orx7cCD+zoIYKbgMSjQEKQQoUNIYjWN3tePJmE5mP+8usYpFNVHwSIgogUjVI2wTi4xiY69iGv7kvlpG7+PanBNdyzNnxofDnkosYgKCUpY0dEkEKFDSGI1jd7XjyZhOZj/vLrGKRTVR8EiIKIFI1SNsE4uMYmOvYhr+5L5aRu/j2pwTXcszZ8aHw55KLGICglKWNHRiAoJSljR0aCAjMkuDiDhAwIo0BCkEKFDSGI1jd7XjyZhOZj/vLrGKRTVR8EiIKIFI1SNsE4uMYmOvYhr+5L5aRu/j2pwTXcszZ8aHw55KLGICglKWNHRJBChQ0hiNY3e148mYTmY/7y6xikU1UfBIiCiBSNUjbBOLjGJjr2Ia/uS+Wkbv49qcE13LM2fGh8OeSixiAoJSljR0YgKCUpY0d";
        let header_43_data =
            "CtQECpgDCgIICxISY2hhaW4tMS0zOTY1MTkyNTI0GEYiDAjCj/S/BhDcqMv/AipICiDr+FXP2JmUMJId3I6HwCSt+mEpczSZCNKHZe7pgEPNpxIkCAESIGRbMklUq3kyVREQ9dyzGEsZaB6U3SE1+qKF6vMVPBZJMiD08ju+8/EvUt35ojhJJ734jMbD6TOXAMHQ/QJmTBU1Tzog47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFVCINcW8oxHV55BXwPjDwGa1MD56V3MRivoxwHwmFs+txiFSiDXFvKMR1eeQV8D4w8BmtTA+eldzEYr6McB8JhbPrcYhVIgBICRvH3cKD93v7+R1zxE2ljD34qcvIZ0Bdi389qtoi9aIBMBRFTxiVtTxMkUUCa1ipwhwQ9bRuCkCY3PTCP8I0g9YiCypUT2NlQSsXCF65gTxHYNKw8Y+EW6uwgBljV9Ppu37Wog47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFVyFDSGI1jd7XjyZhOZj/vLrGKRTVR8ErYBCEYaSAogS7vnIE846QA5P3nSpjjS0G02m6y8bU0f2HwHfC8S2kMSJAgBEiBb3+vMlGIq7a/CeeqfsZ7H6Tzc5OrlmyGBNOqFr2/kjCJoCAISFDSGI1jd7XjyZhOZj/vLrGKRTVR8GgwIw4/0vwYQyarMhQMiQMRsYYTc3AFAJSL3AQ1BcEJlAwo9YYtZNfHr/dISMf8h6b0rZit+SzyIloxFp4t1NY4hnmwvNB/omqZeo3YQlQcSjQEKQQoUNIYjWN3tePJmE5mP+8usYpFNVHwSIgogUjVI2wTi4xiY69iGv7kvlpG7+PanBNdyzNnxofDnkosYgKCUpY0dEkEKFDSGI1jd7XjyZhOZj/vLrGKRTVR8EiIKIFI1SNsE4uMYmOvYhr+5L5aRu/j2pwTXcszZ8aHw55KLGICglKWNHRiAoJSljR0aCAjMkuDiDhAwIo0BCkEKFDSGI1jd7XjyZhOZj/vLrGKRTVR8EiIKIFI1SNsE4uMYmOvYhr+5L5aRu/j2pwTXcszZ8aHw55KLGICglKWNHRJBChQ0hiNY3e148mYTmY/7y6xikU1UfBIiCiBSNUjbBOLjGJjr2Ia/uS+Wkbv49qcE13LM2fGh8OeSixiAoJSljR0YgKCUpY0d";

        let header_31_bytes = base64::decode_byte_array(header_31_data);
        let header_43_bytes = base64::decode_byte_array(header_43_data);

        let header_31 = ProtoCodecImpl::decode::<LcHeader>(header_31_bytes.span()).unwrap();
        let header_43 = ProtoCodecImpl::decode::<LcHeader>(header_43_bytes.span()).unwrap();

        assert_eq!(@header_31.signed_header.header.height, @69);
        assert_eq!(@header_43.signed_header.header.height, @70);

        assert_eq!(
            @header_31.signed_header.header.chain_id, @header_43.signed_header.header.chain_id,
        );
        assert_lt!(@header_31.signed_header.header.time, @header_43.signed_header.header.time);

        assert_eq!(
            @header_31.signed_header.header.next_validators_hash,
            @header_43.signed_header.header.validators_hash,
        );

        (header_31, header_43)
    };

    let trusting_period = Duration { seconds: 1209600, nanos: 0 };

    let clock_drift = Duration { seconds: 3, nanos: 0 };

    let options = Options { trust_threshold: TWO_THIRDS, trusting_period, clock_drift };

    let now = Timestamp {
        // header is submitted 30 seconds later
        seconds: header_b.signed_header.header.time.seconds + 30,
        nanos: header_b.signed_header.header.time.nanos,
    };

    let trusted_block_state = TrustedBlockState {
        chain_id: header_a.signed_header.header.chain_id,
        header_time: header_a.signed_header.header.time,
        height: header_a.signed_header.header.height,
        next_validators: header_b.validator_set.clone(), // full validator_set is in future header
        next_validators_hash: header_a.signed_header.header.next_validators_hash,
    };

    let untrusted_block_state = UntrustedBlockState {
        signed_header: header_b.signed_header,
        validators: header_b.validator_set,
        next_validators: header_b.trusted_validator_set,
    };

    let mut signature_hints = array![
        array![
            0x14, 0x22f7112ecb3a47d3238afb43, 0xb83b38c8070e7b21a867f3ae, 0x68491d01f77894a1, 0x0,
            0x45d0f6d9198cf8ce61749971, 0x7994f31599884b3cca7e0692, 0x49df1dcf8cfef219, 0x0,
            0x2c9ea4855782ea5fc65a920825506cf5, 0x123fc8169734da3c00bc1eed8b13d7523,
            0xc7fc8cb0556019b412452cf5, 0x5688bf20bb7c2de59a54c942, 0x270d457eef940b5e, 0x0,
            0x88927936e0e2d323a3137546, 0x73bb16847286eb43aa43d28a, 0xf159846ac319024, 0x0,
            0x3b5a662b71b401b7a7e093f26066fbce, 0x12a9be48a739410bf8c033ca5c2debae1,
            0x56ac741e87c6845ef7d1cc6fea40494, 0x3b0e8535deec3e40d687ee38f3b05a83,
            0x22a3999ebaf69753199a35c93954c911, 0x6590c8fc904594984073b2ce15394456,
        ],
    ];

    verify_update_header(
        untrusted_block_state, trusted_block_state, options, now, array![].span(), signature_hints,
    );
}
