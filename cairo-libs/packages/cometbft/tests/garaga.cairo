use alexandria_math::ed25519::verify_signature;
use garaga::signatures::eddsa_25519::{
    EdDSASignature, EdDSASignatureWithHint, is_valid_eddsa_signature,
};
use ibc_utils::numeric::reverse_u256;

#[test]
fn test_garaga_fixture_2() {
    let serialized_input = [
        0x4025645f0b820e72b8cad4f0a909a092, 0xda69dbeb232276b38f3f5016547bb2a2,
        0x8c1df1d013368f456e99153ee4c15a08, 0xcbb1216290db0ee2a30b4ae2e7b38,
        0xbc7e1b4da70ab7925a8943e8c317403d, 0xc66f42af155cdc08c96c42ecf2c989c, 0x1, 0x72, 0x14,
        0x6de5e78642ca6272fe226721, 0x7cf40fc75418b3adeb9919d6, 0x31933ca322b3f216, 0x0,
        0x8d5eb7a4c911c6bf43646d05, 0xa3967ce05a17992c52dadd11, 0x281dba6b1a1f027a, 0x0,
        0x43552bb883869c4f088ee6bb4efc13f, 0x2f05da07762de80585670421211da862,
        0xe3b67a511729d95315b524b4, 0x603c857b87f02c26d0beb4cc, 0x5c9f3a5f13f9a7b9, 0x0,
        0xefd05d9cb95fb707a155df23, 0xc19b0542ce180a2a8576b067, 0x7b59aabaa98efd62, 0x0,
        0x28f052ef060e9ddba0b768af28e60013, 0x1f1e5de2ee6845d3ff39e22a609c226,
        0x14fecc2165ca5cee9eee19fe4d2c1, 0x157f7361c577aad36f67ed33e38dc7be,
        0x8085f91fb6a5096f244ae01e57de43ae, 0x74ad28205b4f384bc0813e6585864e52,
    ];

    let mut span = serialized_input.span();
    let input = Serde::deserialize(ref span).unwrap();

    let EdDSASignatureWithHint {
        signature: EdDSASignature { Ry_twisted, s, Py_twisted, msg }, ..,
    } = input.clone();

    assert(is_valid_eddsa_signature(input), 'invalid garaga ed25519');

    assert(
        verify_signature(
            msg, [reverse_u256(Ry_twisted), reverse_u256(s)].span(), reverse_u256(Py_twisted),
        ),
        'invalid alexandria ed25519',
    );
}
