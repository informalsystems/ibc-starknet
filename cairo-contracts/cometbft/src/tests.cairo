use protobuf::types::message::{ProtoCodecImpl};
use protobuf::hex::decode as hex_decode;
use cometbft::light_client::Header;

#[test]
fn test_tm_header_decode() {
    let hex =
        "0ad0030ab4010a02080b12066d6f636b2d301840220608b19bc39d064220e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf984a20e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf985220e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf985a20d604111b0a413fcbf889ee1bd912850802df9a69f53633539cfcb6333efe596b7214a6e7b6810df8120580f2a81710e228f454f99c97129602084010011a480a20c1b852e432ccf60cdacc28925ac4adee083db8f57b6a249991507983c52eaab6122408011220c1b852e432ccf60cdacc28925ac4adee083db8f57b6a249991507983c52eaab6226208021214a6e7b6810df8120580f2a81710e228f454f99c971a0608b19bc39d062240d0232043bda001016db9fc6584baacadff831d9d11e39c98d8dd5892eedcb5db595406d77118a5853aee43cca58a7b7165ffb4c0a5285a45284e5d0ef94d1109226208021214c7832263600476fd6ff4c5cb0a86080d0e5f48b21a0608b19bc39d0622402a6bd24055a10028bc9fc31e8ae3a0cef8ae67508844565ee1d457d392bc40321b8f0bad9476ae6d9d04d53dc27758a4b79e9d82871d912247521dab6a402b03127e0a3c0a14a6e7b6810df8120580f2a81710e228f454f99c9712220a2050c4a5871ad3379f2879d12cef750d1211633283a9c3730238e6ddf084db4c8a18320a3c0a14c7832263600476fd6ff4c5cb0a86080d0e5f48b212220a20ebe80b7cadea277ac05fb85c7164fe15ebd6873c4a74b3296a462a1026fd9b0f183218641a021040227e0a3c0a14a6e7b6810df8120580f2a81710e228f454f99c9712220a2050c4a5871ad3379f2879d12cef750d1211633283a9c3730238e6ddf084db4c8a18320a3c0a14c7832263600476fd6ff4c5cb0a86080d0e5f48b212220a20ebe80b7cadea277ac05fb85c7164fe15ebd6873c4a74b3296a462a1026fd9b0f18321864";
    let bytes = hex_decode(@hex);
    let header = ProtoCodecImpl::decode::<Header>(@bytes);
    let bytes2 = ProtoCodecImpl::encode(@header);
    assert_eq!(bytes, bytes2, "header encode/decode mismatch");
}
