#[starknet::interface]
pub trait IEd25519<TContractState> {
    fn verify_signature(
        self: @TContractState,
        msg: Array<u8>,
        signature: [u256; 2],
        public_key: u256,
        hints: Array<felt252>,
    );
}

#[starknet::component]
pub mod AlexandriaEd25519LibComponent {
    use alexandria_math::ed25519::verify_signature;
    use super::*;

    #[storage]
    pub struct Storage {}

    #[embeddable_as(AlexandriaEd25519Lib)]
    impl AlexandriaEd25519LibImpl<
        TContractState, +HasComponent<TContractState>,
    > of IEd25519<ComponentState<TContractState>> {
        fn verify_signature(
            self: @ComponentState<TContractState>,
            msg: Array<u8>,
            signature: [u256; 2],
            public_key: u256,
            hints: Array<felt252>,
        ) {
            assert(
                verify_signature(msg.span(), signature.span(), public_key),
                'invalid alexandria ed25519',
            )
        }
    }
}

#[starknet::component]
pub mod GaragaEd25519LibComponent {
    use garaga::signatures::eddsa_25519::{
        EdDSASignature, EdDSASignatureWithHint, is_valid_eddsa_signature,
    };
    use super::*;

    #[storage]
    pub struct Storage {}

    #[embeddable_as(GaragaEd25519Lib)]
    impl GaragaEd25519LibImpl<
        TContractState, +HasComponent<TContractState>,
    > of IEd25519<ComponentState<TContractState>> {
        fn verify_signature(
            self: @ComponentState<TContractState>,
            msg: Array<u8>,
            signature: [u256; 2],
            public_key: u256,
            hints: Array<felt252>,
        ) {
            let [sign_r, sign_s] = signature;

            let signature = EdDSASignature {
                Ry_twisted: sign_r, s: sign_s, Py_twisted: public_key, msg: msg.span(),
            };

            let mut hints_span = hints.span();

            let (msm_hint, sqrt_Rx_hint, sqrt_Px_hint) = Serde::deserialize(ref hints_span)
                .unwrap();

            let signature_with_hint = EdDSASignatureWithHint {
                signature, msm_hint, sqrt_Rx_hint, sqrt_Px_hint,
            };

            assert(is_valid_eddsa_signature(signature_with_hint), 'invalid garaga ed25519')
        }
    }
}
