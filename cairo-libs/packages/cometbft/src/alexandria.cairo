use alexandria_math::ed25519::verify_signature;
use cometbft::errors::CometErrors;
use cometbft::types::Ed25519;
use cometbft::utils::SpanU8TryIntoU256;

#[derive(Drop)]
pub struct AlexandriaEd25519 {}

pub impl AlexandriaEd25519Impl of Ed25519<AlexandriaEd25519> {
    fn verify(
        self: @AlexandriaEd25519, pub_key: Span<u8>, msg: Span<u8>, signature: Span<u8>,
    ) -> bool {
        assert(signature.len() == 64, CometErrors::INVALID_SIGNATURE_LENGTH);
        assert(pub_key.len() == 32, CometErrors::INVALID_PUBKEY_LENGTH);

        let r_sign = signature.slice(0, 32).try_into().unwrap(); // Never fails as length is 32.
        let s_sign = signature.slice(32, 32).try_into().unwrap(); // Never fails as length is 32.
        let pubkey = pub_key.try_into().unwrap(); // Never fails as length is 32.

        verify_signature(msg, array![r_sign, s_sign].span(), pubkey)
    }
}
