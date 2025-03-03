pub mod hades;
pub mod poseidon_3;

use hades::HADES_PERM_3;
use starknet_types_core::felt::Felt;

// References:
// https://docs.starknet.io/architecture-and-concepts/cryptography/hash-functions
// https://github.com/starkware-industries/poseidon
// https://github.com/starkware-libs/cairo/blob/master/corelib/src/poseidon.cairo
// https://github.com/starkware-libs/cairo-lang/blob/master/src/starkware/cairo/common/poseidon_hash.py
// https://github.com/starkware-libs/cairo-lang/blob/master/src/starkware/cairo/common/poseidon_utils.py

pub struct Poseidon3Hasher {
    pub state: [Felt; 3],
    pub odd: bool,
}

impl Default for Poseidon3Hasher {
    fn default() -> Self {
        Self {
            state: [Felt::ZERO; 3],
            odd: false,
        }
    }
}

impl Poseidon3Hasher {
    pub fn write(mut self, value: Felt) -> Self {
        if self.odd {
            self.state = HADES_PERM_3.hades_permutation([
                self.state[0],
                self.state[1] + value,
                self.state[2],
            ]);
            self.odd = false;
        } else {
            self.state[0] += value;
            self.odd = true;
        }
        self
    }

    pub fn finish(self) -> Felt {
        if self.odd {
            HADES_PERM_3.hades_permutation([
                self.state[0],
                self.state[1] + Felt::ONE,
                self.state[2],
            ])[0]
        } else {
            HADES_PERM_3.hades_permutation([
                self.state[0] + Felt::ONE,
                self.state[1],
                self.state[2],
            ])[0]
        }
    }

    pub fn digest(span: &[Felt]) -> Felt {
        let mut state = [Felt::ZERO; 3];

        for chunk in span.chunks(2) {
            match chunk {
                [x, y] => {
                    state = HADES_PERM_3.hades_permutation([state[0] + x, state[1] + y, state[2]]);
                }
                [x] => {
                    return Self {
                        state: [state[0] + x, state[1], state[2]],
                        odd: true,
                    }
                    .finish()
                }
                _ => unreachable!(),
            }
        }

        Self { state, odd: false }.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poseidon_digest() {
        // https://github.com/starkware-libs/cairo/blob/dff35c09bfaa1ae0969c48ce4e103bad46d5fe50/corelib/src/poseidon.cairo#L128

        let span = [Felt::ONE, Felt::TWO];
        let hash = Poseidon3Hasher::digest(&span);

        let expected_hex = "0x0371cb6995ea5e7effcd2e174de264b5b407027a75a231a70c2c8d196107f0e7";

        assert_eq!(hash, Felt::from_hex(expected_hex).unwrap());
    }

    #[test]
    fn test_poseidon_write_finish() {
        // https://github.com/starkware-libs/cairo/blob/dff35c09bfaa1ae0969c48ce4e103bad46d5fe50/corelib/src/poseidon.cairo#L99

        let hash = Poseidon3Hasher::default()
            .write(Felt::ONE)
            .write(Felt::TWO)
            .finish();

        let expected_hex = "0x0371cb6995ea5e7effcd2e174de264b5b407027a75a231a70c2c8d196107f0e7";

        assert_eq!(hash, Felt::from_hex(expected_hex).unwrap());
    }

    #[test]
    fn test_poseidon_equivalent() {
        let data: [Felt; 20] = core::array::from_fn(Felt::from);

        for i in 0..data.len() {
            let mut hasher = Poseidon3Hasher::default();

            let current_data = &data[..=i];

            for felt in current_data {
                hasher = hasher.write(*felt);
            }

            let update_hash = hasher.finish();

            let digest_hash = Poseidon3Hasher::digest(current_data);

            assert_eq!(update_hash, digest_hash);
        }
    }
}
