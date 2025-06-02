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

#[starknet::contract]
pub mod IAlexandriaEd25519Lib {
    use alexandria_math::ed25519::verify_signature;
    use super::*;

    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl IAlexandriaEd25519Impl of IEd25519<ContractState> {
        fn verify_signature(
            self: @ContractState,
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

#[starknet::contract]
pub mod IGaragaEd25519Lib {
    use garaga::signatures::eddsa_25519::{
        EdDSASignature, EdDSASignatureWithHint, is_valid_eddsa_signature,
    };
    use super::*;

    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl IGaragaEd25519Impl of IEd25519<ContractState> {
        fn verify_signature(
            self: @ContractState,
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
