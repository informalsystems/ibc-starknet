use alexandria_math::ed25519::verify_signature;
use garaga::signatures::eddsa_25519::{
    EdDSASignature, EdDSASignatureWithHint, is_valid_eddsa_signature,
};
use ibc_utils::bytes::{U256AsBigEndian, U256AsLittleEndian};
use crate::errors::CometErrors;

pub trait Ed25519Verifier {
    fn assert_signature(
        msg: Span<u8>, signature: Span<u8>, public_key: Span<u8>, hints: Span<felt252>,
    );
}

pub impl AlexandriaEd25519Verifier of Ed25519Verifier {
    fn assert_signature(
        msg: Span<u8>, signature: Span<u8>, public_key: Span<u8>, hints: Span<felt252>,
    ) {
        assert(signature.len() == 64, CometErrors::INVALID_SIGNATURE_LENGTH);
        assert(public_key.len() == 32, CometErrors::INVALID_PUBKEY_LENGTH);

        let r_sign: u256 = U256AsBigEndian::from_bytes(signature.slice(0, 32)).unwrap();
        let s_sign: u256 = U256AsBigEndian::from_bytes(signature.slice(32, 64)).unwrap();
        let pubkey: u256 = U256AsBigEndian::from_bytes(public_key).unwrap();

        assert(verify_signature(msg, [r_sign, s_sign].span(), pubkey), 'invalid alexandria ed25519')
    }
}

pub impl GaragaEd25519Verifier of Ed25519Verifier {
    fn assert_signature(
        msg: Span<u8>, signature: Span<u8>, public_key: Span<u8>, mut hints: Span<felt252>,
    ) {
        assert(signature.len() == 64, CometErrors::INVALID_SIGNATURE_LENGTH);
        assert(public_key.len() == 32, CometErrors::INVALID_PUBKEY_LENGTH);

        let Ry_twisted: u256 = U256AsLittleEndian::from_bytes(signature.slice(0, 32))
            .unwrap();
        let s: u256 = U256AsLittleEndian::from_bytes(signature.slice(32, 64)).unwrap();
        let Py_twisted: u256 = U256AsLittleEndian::from_bytes(public_key).unwrap();

        let signature = EdDSASignature { Ry_twisted, s, Py_twisted, msg };

        let (msm_hint, sqrt_Rx_hint, sqrt_Px_hint) = Serde::deserialize(ref hints).unwrap();

        let signature_with_hint = EdDSASignatureWithHint {
            signature, msm_hint, sqrt_Rx_hint, sqrt_Px_hint,
        };

        assert(is_valid_eddsa_signature(signature_with_hint), 'invalid garaga ed25519')
    }
}

