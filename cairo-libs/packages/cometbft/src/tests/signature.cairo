use cometbft::utils::SpanU8TryIntoU256;
use cometbft::types::{PublicKey, PublicKeyTrait, Sum};

#[derive(Drop, Debug)]
pub struct TestData {
    pub pubkey: Array<u8>,
    pub msg: Array<u8>,
    pub signature: Array<u8>,
}

// NOTE: Ed25519 test cases are from https://datatracker.ietf.org/doc/html/rfc8032#section-7.1,
// identical to the test vectors in
// [`tendermint-rs`](https://github.com/informalsystems/tendermint-rs/blob/6cc391c80ae988615508bd87285571ba130b604c/tendermint/src/crypto/default/signature.rs#L43)

pub fn ed25519_case_1() -> TestData {
    TestData {
        pubkey: array![
            0xd7,
            0x5a,
            0x98,
            0x01,
            0x82,
            0xb1,
            0x0a,
            0xb7,
            0xd5,
            0x4b,
            0xfe,
            0xd3,
            0xc9,
            0x64,
            0x07,
            0x3a,
            0x0e,
            0xe1,
            0x72,
            0xf3,
            0xda,
            0xa6,
            0x23,
            0x25,
            0xaf,
            0x02,
            0x1a,
            0x68,
            0xf7,
            0x07,
            0x51,
            0x1a,
        ],
        msg: array![],
        signature: array![
            0xe5,
            0x56,
            0x43,
            0x00,
            0xc3,
            0x60,
            0xac,
            0x72,
            0x90,
            0x86,
            0xe2,
            0xcc,
            0x80,
            0x6e,
            0x82,
            0x8a,
            0x84,
            0x87,
            0x7f,
            0x1e,
            0xb8,
            0xe5,
            0xd9,
            0x74,
            0xd8,
            0x73,
            0xe0,
            0x65,
            0x22,
            0x49,
            0x01,
            0x55,
            0x5f,
            0xb8,
            0x82,
            0x15,
            0x90,
            0xa3,
            0x3b,
            0xac,
            0xc6,
            0x1e,
            0x39,
            0x70,
            0x1c,
            0xf9,
            0xb4,
            0x6b,
            0xd2,
            0x5b,
            0xf5,
            0xf0,
            0x59,
            0x5b,
            0xbe,
            0x24,
            0x65,
            0x51,
            0x41,
            0x43,
            0x8e,
            0x7a,
            0x10,
            0x0b,
        ],
    }
}

pub fn ed25519_case_2() -> TestData {
    TestData {
        pubkey: array![
            0x3d,
            0x40,
            0x17,
            0xc3,
            0xe8,
            0x43,
            0x89,
            0x5a,
            0x92,
            0xb7,
            0x0a,
            0xa7,
            0x4d,
            0x1b,
            0x7e,
            0xbc,
            0x9c,
            0x98,
            0x2c,
            0xcf,
            0x2e,
            0xc4,
            0x96,
            0x8c,
            0xc0,
            0xcd,
            0x55,
            0xf1,
            0x2a,
            0xf4,
            0x66,
            0x0c,
        ],
        msg: array![0x72],
        signature: array![
            0x92,
            0xa0,
            0x09,
            0xa9,
            0xf0,
            0xd4,
            0xca,
            0xb8,
            0x72,
            0x0e,
            0x82,
            0x0b,
            0x5f,
            0x64,
            0x25,
            0x40,
            0xa2,
            0xb2,
            0x7b,
            0x54,
            0x16,
            0x50,
            0x3f,
            0x8f,
            0xb3,
            0x76,
            0x22,
            0x23,
            0xeb,
            0xdb,
            0x69,
            0xda,
            0x08,
            0x5a,
            0xc1,
            0xe4,
            0x3e,
            0x15,
            0x99,
            0x6e,
            0x45,
            0x8f,
            0x36,
            0x13,
            0xd0,
            0xf1,
            0x1d,
            0x8c,
            0x38,
            0x7b,
            0x2e,
            0xae,
            0xb4,
            0x30,
            0x2a,
            0xee,
            0xb0,
            0x0d,
            0x29,
            0x16,
            0x12,
            0xbb,
            0x0c,
            0x00,
        ],
    }
}

pub fn ed25519_case_3() -> TestData {
    TestData {
        pubkey: array![
            0xfc,
            0x51,
            0xcd,
            0x8e,
            0x62,
            0x18,
            0xa1,
            0xa3,
            0x8d,
            0xa4,
            0x7e,
            0xd0,
            0x02,
            0x30,
            0xf0,
            0x58,
            0x08,
            0x16,
            0xed,
            0x13,
            0xba,
            0x33,
            0x03,
            0xac,
            0x5d,
            0xeb,
            0x91,
            0x15,
            0x48,
            0x90,
            0x80,
            0x25,
        ],
        msg: array![0xaf, 0x82],
        signature: array![
            0x62,
            0x91,
            0xd6,
            0x57,
            0xde,
            0xec,
            0x24,
            0x02,
            0x48,
            0x27,
            0xe6,
            0x9c,
            0x3a,
            0xbe,
            0x01,
            0xa3,
            0x0c,
            0xe5,
            0x48,
            0xa2,
            0x84,
            0x74,
            0x3a,
            0x44,
            0x5e,
            0x36,
            0x80,
            0xd7,
            0xdb,
            0x5a,
            0xc3,
            0xac,
            0x18,
            0xff,
            0x9b,
            0x53,
            0x8d,
            0x16,
            0xf2,
            0x90,
            0xae,
            0x67,
            0xf7,
            0x60,
            0x98,
            0x4d,
            0xc6,
            0x59,
            0x4a,
            0x7c,
            0x15,
            0xe9,
            0x71,
            0x6e,
            0xd2,
            0x8d,
            0xc0,
            0x27,
            0xbe,
            0xce,
            0xea,
            0x1e,
            0xc4,
            0x0a,
        ],
    }
}

pub fn ed25519_case_1024() -> TestData {
    TestData {
        pubkey: array![
            0x27,
            0x81,
            0x17,
            0xfc,
            0x14,
            0x4c,
            0x72,
            0x34,
            0x0f,
            0x67,
            0xd0,
            0xf2,
            0x31,
            0x6e,
            0x83,
            0x86,
            0xce,
            0xff,
            0xbf,
            0x2b,
            0x24,
            0x28,
            0xc9,
            0xc5,
            0x1f,
            0xef,
            0x7c,
            0x59,
            0x7f,
            0x1d,
            0x42,
            0x6e,
        ],
        msg: array![
            0x08,
            0xb8,
            0xb2,
            0xb7,
            0x33,
            0x42,
            0x42,
            0x43,
            0x76,
            0x0f,
            0xe4,
            0x26,
            0xa4,
            0xb5,
            0x49,
            0x08,
            0x63,
            0x21,
            0x10,
            0xa6,
            0x6c,
            0x2f,
            0x65,
            0x91,
            0xea,
            0xbd,
            0x33,
            0x45,
            0xe3,
            0xe4,
            0xeb,
            0x98,
            0xfa,
            0x6e,
            0x26,
            0x4b,
            0xf0,
            0x9e,
            0xfe,
            0x12,
            0xee,
            0x50,
            0xf8,
            0xf5,
            0x4e,
            0x9f,
            0x77,
            0xb1,
            0xe3,
            0x55,
            0xf6,
            0xc5,
            0x05,
            0x44,
            0xe2,
            0x3f,
            0xb1,
            0x43,
            0x3d,
            0xdf,
            0x73,
            0xbe,
            0x84,
            0xd8,
            0x79,
            0xde,
            0x7c,
            0x00,
            0x46,
            0xdc,
            0x49,
            0x96,
            0xd9,
            0xe7,
            0x73,
            0xf4,
            0xbc,
            0x9e,
            0xfe,
            0x57,
            0x38,
            0x82,
            0x9a,
            0xdb,
            0x26,
            0xc8,
            0x1b,
            0x37,
            0xc9,
            0x3a,
            0x1b,
            0x27,
            0x0b,
            0x20,
            0x32,
            0x9d,
            0x65,
            0x86,
            0x75,
            0xfc,
            0x6e,
            0xa5,
            0x34,
            0xe0,
            0x81,
            0x0a,
            0x44,
            0x32,
            0x82,
            0x6b,
            0xf5,
            0x8c,
            0x94,
            0x1e,
            0xfb,
            0x65,
            0xd5,
            0x7a,
            0x33,
            0x8b,
            0xbd,
            0x2e,
            0x26,
            0x64,
            0x0f,
            0x89,
            0xff,
            0xbc,
            0x1a,
            0x85,
            0x8e,
            0xfc,
            0xb8,
            0x55,
            0x0e,
            0xe3,
            0xa5,
            0xe1,
            0x99,
            0x8b,
            0xd1,
            0x77,
            0xe9,
            0x3a,
            0x73,
            0x63,
            0xc3,
            0x44,
            0xfe,
            0x6b,
            0x19,
            0x9e,
            0xe5,
            0xd0,
            0x2e,
            0x82,
            0xd5,
            0x22,
            0xc4,
            0xfe,
            0xba,
            0x15,
            0x45,
            0x2f,
            0x80,
            0x28,
            0x8a,
            0x82,
            0x1a,
            0x57,
            0x91,
            0x16,
            0xec,
            0x6d,
            0xad,
            0x2b,
            0x3b,
            0x31,
            0x0d,
            0xa9,
            0x03,
            0x40,
            0x1a,
            0xa6,
            0x21,
            0x00,
            0xab,
            0x5d,
            0x1a,
            0x36,
            0x55,
            0x3e,
            0x06,
            0x20,
            0x3b,
            0x33,
            0x89,
            0x0c,
            0xc9,
            0xb8,
            0x32,
            0xf7,
            0x9e,
            0xf8,
            0x05,
            0x60,
            0xcc,
            0xb9,
            0xa3,
            0x9c,
            0xe7,
            0x67,
            0x96,
            0x7e,
            0xd6,
            0x28,
            0xc6,
            0xad,
            0x57,
            0x3c,
            0xb1,
            0x16,
            0xdb,
            0xef,
            0xef,
            0xd7,
            0x54,
            0x99,
            0xda,
            0x96,
            0xbd,
            0x68,
            0xa8,
            0xa9,
            0x7b,
            0x92,
            0x8a,
            0x8b,
            0xbc,
            0x10,
            0x3b,
            0x66,
            0x21,
            0xfc,
            0xde,
            0x2b,
            0xec,
            0xa1,
            0x23,
            0x1d,
            0x20,
            0x6b,
            0xe6,
            0xcd,
            0x9e,
            0xc7,
            0xaf,
            0xf6,
            0xf6,
            0xc9,
            0x4f,
            0xcd,
            0x72,
            0x04,
            0xed,
            0x34,
            0x55,
            0xc6,
            0x8c,
            0x83,
            0xf4,
            0xa4,
            0x1d,
            0xa4,
            0xaf,
            0x2b,
            0x74,
            0xef,
            0x5c,
            0x53,
            0xf1,
            0xd8,
            0xac,
            0x70,
            0xbd,
            0xcb,
            0x7e,
            0xd1,
            0x85,
            0xce,
            0x81,
            0xbd,
            0x84,
            0x35,
            0x9d,
            0x44,
            0x25,
            0x4d,
            0x95,
            0x62,
            0x9e,
            0x98,
            0x55,
            0xa9,
            0x4a,
            0x7c,
            0x19,
            0x58,
            0xd1,
            0xf8,
            0xad,
            0xa5,
            0xd0,
            0x53,
            0x2e,
            0xd8,
            0xa5,
            0xaa,
            0x3f,
            0xb2,
            0xd1,
            0x7b,
            0xa7,
            0x0e,
            0xb6,
            0x24,
            0x8e,
            0x59,
            0x4e,
            0x1a,
            0x22,
            0x97,
            0xac,
            0xbb,
            0xb3,
            0x9d,
            0x50,
            0x2f,
            0x1a,
            0x8c,
            0x6e,
            0xb6,
            0xf1,
            0xce,
            0x22,
            0xb3,
            0xde,
            0x1a,
            0x1f,
            0x40,
            0xcc,
            0x24,
            0x55,
            0x41,
            0x19,
            0xa8,
            0x31,
            0xa9,
            0xaa,
            0xd6,
            0x07,
            0x9c,
            0xad,
            0x88,
            0x42,
            0x5d,
            0xe6,
            0xbd,
            0xe1,
            0xa9,
            0x18,
            0x7e,
            0xbb,
            0x60,
            0x92,
            0xcf,
            0x67,
            0xbf,
            0x2b,
            0x13,
            0xfd,
            0x65,
            0xf2,
            0x70,
            0x88,
            0xd7,
            0x8b,
            0x7e,
            0x88,
            0x3c,
            0x87,
            0x59,
            0xd2,
            0xc4,
            0xf5,
            0xc6,
            0x5a,
            0xdb,
            0x75,
            0x53,
            0x87,
            0x8a,
            0xd5,
            0x75,
            0xf9,
            0xfa,
            0xd8,
            0x78,
            0xe8,
            0x0a,
            0x0c,
            0x9b,
            0xa6,
            0x3b,
            0xcb,
            0xcc,
            0x27,
            0x32,
            0xe6,
            0x94,
            0x85,
            0xbb,
            0xc9,
            0xc9,
            0x0b,
            0xfb,
            0xd6,
            0x24,
            0x81,
            0xd9,
            0x08,
            0x9b,
            0xec,
            0xcf,
            0x80,
            0xcf,
            0xe2,
            0xdf,
            0x16,
            0xa2,
            0xcf,
            0x65,
            0xbd,
            0x92,
            0xdd,
            0x59,
            0x7b,
            0x07,
            0x07,
            0xe0,
            0x91,
            0x7a,
            0xf4,
            0x8b,
            0xbb,
            0x75,
            0xfe,
            0xd4,
            0x13,
            0xd2,
            0x38,
            0xf5,
            0x55,
            0x5a,
            0x7a,
            0x56,
            0x9d,
            0x80,
            0xc3,
            0x41,
            0x4a,
            0x8d,
            0x08,
            0x59,
            0xdc,
            0x65,
            0xa4,
            0x61,
            0x28,
            0xba,
            0xb2,
            0x7a,
            0xf8,
            0x7a,
            0x71,
            0x31,
            0x4f,
            0x31,
            0x8c,
            0x78,
            0x2b,
            0x23,
            0xeb,
            0xfe,
            0x80,
            0x8b,
            0x82,
            0xb0,
            0xce,
            0x26,
            0x40,
            0x1d,
            0x2e,
            0x22,
            0xf0,
            0x4d,
            0x83,
            0xd1,
            0x25,
            0x5d,
            0xc5,
            0x1a,
            0xdd,
            0xd3,
            0xb7,
            0x5a,
            0x2b,
            0x1a,
            0xe0,
            0x78,
            0x45,
            0x04,
            0xdf,
            0x54,
            0x3a,
            0xf8,
            0x96,
            0x9b,
            0xe3,
            0xea,
            0x70,
            0x82,
            0xff,
            0x7f,
            0xc9,
            0x88,
            0x8c,
            0x14,
            0x4d,
            0xa2,
            0xaf,
            0x58,
            0x42,
            0x9e,
            0xc9,
            0x60,
            0x31,
            0xdb,
            0xca,
            0xd3,
            0xda,
            0xd9,
            0xaf,
            0x0d,
            0xcb,
            0xaa,
            0xaf,
            0x26,
            0x8c,
            0xb8,
            0xfc,
            0xff,
            0xea,
            0xd9,
            0x4f,
            0x3c,
            0x7c,
            0xa4,
            0x95,
            0xe0,
            0x56,
            0xa9,
            0xb4,
            0x7a,
            0xcd,
            0xb7,
            0x51,
            0xfb,
            0x73,
            0xe6,
            0x66,
            0xc6,
            0xc6,
            0x55,
            0xad,
            0xe8,
            0x29,
            0x72,
            0x97,
            0xd0,
            0x7a,
            0xd1,
            0xba,
            0x5e,
            0x43,
            0xf1,
            0xbc,
            0xa3,
            0x23,
            0x01,
            0x65,
            0x13,
            0x39,
            0xe2,
            0x29,
            0x04,
            0xcc,
            0x8c,
            0x42,
            0xf5,
            0x8c,
            0x30,
            0xc0,
            0x4a,
            0xaf,
            0xdb,
            0x03,
            0x8d,
            0xda,
            0x08,
            0x47,
            0xdd,
            0x98,
            0x8d,
            0xcd,
            0xa6,
            0xf3,
            0xbf,
            0xd1,
            0x5c,
            0x4b,
            0x4c,
            0x45,
            0x25,
            0x00,
            0x4a,
            0xa0,
            0x6e,
            0xef,
            0xf8,
            0xca,
            0x61,
            0x78,
            0x3a,
            0xac,
            0xec,
            0x57,
            0xfb,
            0x3d,
            0x1f,
            0x92,
            0xb0,
            0xfe,
            0x2f,
            0xd1,
            0xa8,
            0x5f,
            0x67,
            0x24,
            0x51,
            0x7b,
            0x65,
            0xe6,
            0x14,
            0xad,
            0x68,
            0x08,
            0xd6,
            0xf6,
            0xee,
            0x34,
            0xdf,
            0xf7,
            0x31,
            0x0f,
            0xdc,
            0x82,
            0xae,
            0xbf,
            0xd9,
            0x04,
            0xb0,
            0x1e,
            0x1d,
            0xc5,
            0x4b,
            0x29,
            0x27,
            0x09,
            0x4b,
            0x2d,
            0xb6,
            0x8d,
            0x6f,
            0x90,
            0x3b,
            0x68,
            0x40,
            0x1a,
            0xde,
            0xbf,
            0x5a,
            0x7e,
            0x08,
            0xd7,
            0x8f,
            0xf4,
            0xef,
            0x5d,
            0x63,
            0x65,
            0x3a,
            0x65,
            0x04,
            0x0c,
            0xf9,
            0xbf,
            0xd4,
            0xac,
            0xa7,
            0x98,
            0x4a,
            0x74,
            0xd3,
            0x71,
            0x45,
            0x98,
            0x67,
            0x80,
            0xfc,
            0x0b,
            0x16,
            0xac,
            0x45,
            0x16,
            0x49,
            0xde,
            0x61,
            0x88,
            0xa7,
            0xdb,
            0xdf,
            0x19,
            0x1f,
            0x64,
            0xb5,
            0xfc,
            0x5e,
            0x2a,
            0xb4,
            0x7b,
            0x57,
            0xf7,
            0xf7,
            0x27,
            0x6c,
            0xd4,
            0x19,
            0xc1,
            0x7a,
            0x3c,
            0xa8,
            0xe1,
            0xb9,
            0x39,
            0xae,
            0x49,
            0xe4,
            0x88,
            0xac,
            0xba,
            0x6b,
            0x96,
            0x56,
            0x10,
            0xb5,
            0x48,
            0x01,
            0x09,
            0xc8,
            0xb1,
            0x7b,
            0x80,
            0xe1,
            0xb7,
            0xb7,
            0x50,
            0xdf,
            0xc7,
            0x59,
            0x8d,
            0x5d,
            0x50,
            0x11,
            0xfd,
            0x2d,
            0xcc,
            0x56,
            0x00,
            0xa3,
            0x2e,
            0xf5,
            0xb5,
            0x2a,
            0x1e,
            0xcc,
            0x82,
            0x0e,
            0x30,
            0x8a,
            0xa3,
            0x42,
            0x72,
            0x1a,
            0xac,
            0x09,
            0x43,
            0xbf,
            0x66,
            0x86,
            0xb6,
            0x4b,
            0x25,
            0x79,
            0x37,
            0x65,
            0x04,
            0xcc,
            0xc4,
            0x93,
            0xd9,
            0x7e,
            0x6a,
            0xed,
            0x3f,
            0xb0,
            0xf9,
            0xcd,
            0x71,
            0xa4,
            0x3d,
            0xd4,
            0x97,
            0xf0,
            0x1f,
            0x17,
            0xc0,
            0xe2,
            0xcb,
            0x37,
            0x97,
            0xaa,
            0x2a,
            0x2f,
            0x25,
            0x66,
            0x56,
            0x16,
            0x8e,
            0x6c,
            0x49,
            0x6a,
            0xfc,
            0x5f,
            0xb9,
            0x32,
            0x46,
            0xf6,
            0xb1,
            0x11,
            0x63,
            0x98,
            0xa3,
            0x46,
            0xf1,
            0xa6,
            0x41,
            0xf3,
            0xb0,
            0x41,
            0xe9,
            0x89,
            0xf7,
            0x91,
            0x4f,
            0x90,
            0xcc,
            0x2c,
            0x7f,
            0xff,
            0x35,
            0x78,
            0x76,
            0xe5,
            0x06,
            0xb5,
            0x0d,
            0x33,
            0x4b,
            0xa7,
            0x7c,
            0x22,
            0x5b,
            0xc3,
            0x07,
            0xba,
            0x53,
            0x71,
            0x52,
            0xf3,
            0xf1,
            0x61,
            0x0e,
            0x4e,
            0xaf,
            0xe5,
            0x95,
            0xf6,
            0xd9,
            0xd9,
            0x0d,
            0x11,
            0xfa,
            0xa9,
            0x33,
            0xa1,
            0x5e,
            0xf1,
            0x36,
            0x95,
            0x46,
            0x86,
            0x8a,
            0x7f,
            0x3a,
            0x45,
            0xa9,
            0x67,
            0x68,
            0xd4,
            0x0f,
            0xd9,
            0xd0,
            0x34,
            0x12,
            0xc0,
            0x91,
            0xc6,
            0x31,
            0x5c,
            0xf4,
            0xfd,
            0xe7,
            0xcb,
            0x68,
            0x60,
            0x69,
            0x37,
            0x38,
            0x0d,
            0xb2,
            0xea,
            0xaa,
            0x70,
            0x7b,
            0x4c,
            0x41,
            0x85,
            0xc3,
            0x2e,
            0xdd,
            0xcd,
            0xd3,
            0x06,
            0x70,
            0x5e,
            0x4d,
            0xc1,
            0xff,
            0xc8,
            0x72,
            0xee,
            0xee,
            0x47,
            0x5a,
            0x64,
            0xdf,
            0xac,
            0x86,
            0xab,
            0xa4,
            0x1c,
            0x06,
            0x18,
            0x98,
            0x3f,
            0x87,
            0x41,
            0xc5,
            0xef,
            0x68,
            0xd3,
            0xa1,
            0x01,
            0xe8,
            0xa3,
            0xb8,
            0xca,
            0xc6,
            0x0c,
            0x90,
            0x5c,
            0x15,
            0xfc,
            0x91,
            0x08,
            0x40,
            0xb9,
            0x4c,
            0x00,
            0xa0,
            0xb9,
            0xd0,
        ],
        signature: array![
            0x0a,
            0xab,
            0x4c,
            0x90,
            0x05,
            0x01,
            0xb3,
            0xe2,
            0x4d,
            0x7c,
            0xdf,
            0x46,
            0x63,
            0x32,
            0x6a,
            0x3a,
            0x87,
            0xdf,
            0x5e,
            0x48,
            0x43,
            0xb2,
            0xcb,
            0xdb,
            0x67,
            0xcb,
            0xf6,
            0xe4,
            0x60,
            0xfe,
            0xc3,
            0x50,
            0xaa,
            0x53,
            0x71,
            0xb1,
            0x50,
            0x8f,
            0x9f,
            0x45,
            0x28,
            0xec,
            0xea,
            0x23,
            0xc4,
            0x36,
            0xd9,
            0x4b,
            0x5e,
            0x8f,
            0xcd,
            0x4f,
            0x68,
            0x1e,
            0x30,
            0xa6,
            0xac,
            0x00,
            0xa9,
            0x70,
            0x4a,
            0x18,
            0x8a,
            0x03,
        ],
    }
}

pub fn test_verify_ed25519_signature(data: TestData) {
    let pubkey = PublicKey { sum: Sum::Ed25519(data.pubkey) };
    pubkey.verify(data.msg.span(), data.signature.span());
}

#[test]
fn test_ed25519_case_1() {
    test_verify_ed25519_signature(ed25519_case_1())
}

#[test]
fn test_ed25519_case_2() {
    test_verify_ed25519_signature(ed25519_case_2())
}

#[test]
fn test_ed25519_case_3() {
    test_verify_ed25519_signature(ed25519_case_3())
}

#[test]
fn test_ed25519_case_1024() {
    test_verify_ed25519_signature(ed25519_case_1024())
}
