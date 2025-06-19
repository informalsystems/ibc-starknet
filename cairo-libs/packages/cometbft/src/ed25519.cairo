use alexandria_math::ed25519::verify_signature;
use garaga::signatures::eddsa_25519::{
    EdDSASignature, EdDSASignatureWithHint, is_valid_eddsa_signature,
};
use ibc_utils::numeric::reverse_u256;

pub trait Ed25519Verifier {
    fn assert_signature(
        msg: Span<u8>, signature: [u256; 2], public_key: u256, hints: Span<felt252>,
    );
}

pub impl AlexandriaEd25519Verifier of Ed25519Verifier {
    fn assert_signature(
        msg: Span<u8>, signature: [u256; 2], public_key: u256, hints: Span<felt252>,
    ) {
        assert(verify_signature(msg, signature.span(), public_key), 'invalid alexandria ed25519')
    }
}

pub impl GaragaEd25519Verifier of Ed25519Verifier {
    fn assert_signature(
        msg: Span<u8>, signature: [u256; 2], public_key: u256, mut hints: Span<felt252>,
    ) {
        let [sign_r, sign_s] = signature;

        let signature = EdDSASignature {
            Ry_twisted: reverse_u256(sign_r), s: reverse_u256(sign_s), Py_twisted: reverse_u256(public_key), msg: msg,
        };

        let (msm_hint, sqrt_Rx_hint, sqrt_Px_hint) = Serde::deserialize(ref hints).unwrap();

        let signature_with_hint = EdDSASignatureWithHint {
            signature, msm_hint, sqrt_Rx_hint, sqrt_Px_hint,
        };

        assert(is_valid_eddsa_signature(signature_with_hint), 'invalid garaga ed25519')
    }
}

