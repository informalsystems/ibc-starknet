use alexandria_math::ed25519::verify_signature;
use garaga::signatures::eddsa_25519::{
    EdDSASignature, EdDSASignatureWithHint, is_valid_eddsa_signature,
};
use ibc_utils::bytes::{U256AsBigEndian, U256AsLittleEndian};
use crate::errors::CometErrors;

pub trait Ed25519Verifier {
    fn assert_signature(
        msg: Span<u8>,
        signature: Span<u8>,
        public_key: Span<u8>,
        hints_context: Span<felt252>,
        hints: Span<felt252>,
    );
}

pub impl AlexandriaEd25519Verifier of Ed25519Verifier {
    fn assert_signature(
        msg: Span<u8>,
        signature: Span<u8>,
        public_key: Span<u8>,
        hints_context: Span<felt252>,
        hints: Span<felt252>,
    ) {
        assert(signature.len() == 64, CometErrors::INVALID_SIGNATURE_LENGTH);
        assert(public_key.len() == 32, CometErrors::INVALID_PUBKEY_LENGTH);

        let r_sign: u256 = U256AsBigEndian::from_bytes(signature.slice(0, 32)).unwrap();
        let s_sign: u256 = U256AsBigEndian::from_bytes(signature.slice(32, 32)).unwrap();
        let pubkey: u256 = U256AsBigEndian::from_bytes(public_key).unwrap();

        assert(verify_signature(msg, [r_sign, s_sign].span(), pubkey), 'invalid alexandria ed25519')
    }
}

pub impl GaragaEd25519Verifier of Ed25519Verifier {
    fn assert_signature(
        msg: Span<u8>,
        signature: Span<u8>,
        public_key: Span<u8>,
        hints_context: Span<felt252>,
        mut hints: Span<felt252>,
    ) {
        assert(signature.len() == 64, CometErrors::INVALID_SIGNATURE_LENGTH);
        assert(public_key.len() == 32, CometErrors::INVALID_PUBKEY_LENGTH);

        let Ry_twisted: u256 = U256AsLittleEndian::from_bytes(signature.slice(0, 32)).unwrap();
        let s: u256 = U256AsLittleEndian::from_bytes(signature.slice(32, 32)).unwrap();
        let Py_twisted: u256 = U256AsLittleEndian::from_bytes(public_key).unwrap();

        let signature = EdDSASignature { Ry_twisted, s, Py_twisted, msg };

        let (msm_hint, sqrt_Rx_hint, sqrt_Px_hint) = Serde::deserialize(ref hints).unwrap();

        let signature_with_hint = EdDSASignatureWithHint {
            signature, msm_hint, sqrt_Rx_hint, sqrt_Px_hint,
        };

        assert(is_valid_eddsa_signature(signature_with_hint), 'invalid garaga ed25519')
    }
}

pub impl AttestatorEd25519Verifier of Ed25519Verifier {
    fn assert_signature(
        mut msg: Span<u8>,
        mut signature: Span<u8>,
        mut public_key: Span<u8>,
        mut hints_context: Span<felt252>,
        mut hints: Span<felt252>,
    ) {
        assert(signature.len() == 64, CometErrors::INVALID_SIGNATURE_LENGTH);
        assert(public_key.len() == 32, CometErrors::INVALID_PUBKEY_LENGTH);

        let mut attestator_keys: Array<felt252> = Serde::deserialize(ref hints_context).unwrap();

        // Array((pub_key, r, s))
        let mut attestator_signatures: Array<(felt252, felt252, felt252)> = Serde::deserialize(
            ref hints,
        )
            .unwrap();

        let mut attestation_msg: Array<felt252> = array![msg.len().into()];

        while let Some(byte) = msg.pop_front() {
            attestation_msg.append((*byte).into());
        }

        while let Some(byte) = signature.pop_front() {
            attestation_msg.append((*byte).into());
        }

        while let Some(byte) = public_key.pop_front() {
            attestation_msg.append((*byte).into());
        }

        let mut attestation_msg = attestation_msg.span();

        let attestation_hash = core::poseidon::poseidon_hash_span(attestation_msg);

        let mut attestation_count = 0;

        while let Some((pub_key, r, s)) = attestator_signatures.pop_front() {
            let mut is_trusted = false;

            while let Some(trusted_pub_key) = attestator_keys.pop_front() {
                if trusted_pub_key == pub_key {
                    is_trusted = true;
                    break;
                }
            }

            if !is_trusted {
                continue;
            }

            if !core::ecdsa::check_ecdsa_signature(attestation_hash, pub_key, r, s) {
                continue;
            }

            attestation_count += 1;
        }

        assert(attestation_count * 2 > attestator_keys.len(), 'not enough ed25519 attestations');
    }
}

