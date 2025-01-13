use std::sync::LazyLock;

use starknet::core::types::Felt;

use super::poseidon_3::{FULL_ROUNDS, MDS, PARTIAL_ROUNDS, RATE_PLUS_1};

// References:
// https://github.com/starkware-libs/cairo-lang/blob/master/src/starkware/cairo/common/poseidon_utils.py

pub const FN_NAME: &str = "Hades";

pub static ROUND_CONSTANTS: LazyLock<[Felt; RATE_PLUS_1 * (FULL_ROUNDS + PARTIAL_ROUNDS)]> =
    LazyLock::new(|| {
        core::array::from_fn(|idx| {
            // https://github.com/starkware-libs/cairo-lang/blob/master/src/starkware/cairo/common/poseidon_utils.py#L15
            let value = format!("{}{}", FN_NAME, idx);

            let hash = {
                use sha2::Digest;
                let mut hasher = sha2::Sha256::new();
                hasher.update(value);
                hasher.finalize()
            };

            Felt::from_bytes_be(&hash.into())
        })
    });

pub const HADES_PERM_3: HadesPermutate<RATE_PLUS_1, FULL_ROUNDS, PARTIAL_ROUNDS> =
    HadesPermutate { mds: MDS };

pub struct HadesPermutate<const DIM: usize, const FULL_ROUNDS: usize, const PARTIAL_ROUNDS: usize> {
    pub mds: [[i64; DIM]; DIM],
}

impl<const DIM: usize, const FULL_ROUNDS: usize, const PARTIAL_ROUNDS: usize>
    HadesPermutate<DIM, FULL_ROUNDS, PARTIAL_ROUNDS>
{
    // Perform matrix multiplication in the field.
    fn matrix_multiply(matrix: &[[i64; DIM]; DIM], vector: &[Felt; DIM]) -> [Felt; DIM] {
        matrix.map(|row| {
            row.iter()
                .zip(vector.iter())
                .map(|(&m, v)| Felt::from(m) * v)
                .sum()
        })
    }

    // Perform a single round of the Poseidon hash function.
    fn hades_round(
        &self,
        values: [Felt; DIM],
        is_full_round: bool,
        round_idx: usize,
    ) -> [Felt; DIM] {
        // Add-Round Key
        let mut values = core::array::from_fn(|i| values[i] + ROUND_CONSTANTS[round_idx * DIM + i]);

        // Perform the cube operation (x^3) in the field.
        fn cube(x: Felt) -> Felt {
            x * x * x
        }

        // SubWords
        if is_full_round {
            values = values.map(cube);
        } else {
            values[DIM - 1] = cube(values[DIM - 1]);
        }

        // MixLayer
        Self::matrix_multiply(&self.mds, &values)
    }

    // Perform the full Poseidon permutation.
    pub fn hades_permutation(&self, mut values: [Felt; DIM]) -> [Felt; DIM] {
        let mut round_idx = 0;

        // Apply R_F/2 full rounds
        for _ in 0..(FULL_ROUNDS / 2) {
            values = self.hades_round(values, true, round_idx);
            round_idx += 1;
        }

        // Apply R_P partial rounds
        for _ in 0..PARTIAL_ROUNDS {
            values = self.hades_round(values, false, round_idx);
            round_idx += 1;
        }

        // Apply R_F/2 full rounds
        for _ in 0..(FULL_ROUNDS / 2) {
            values = self.hades_round(values, true, round_idx);
            round_idx += 1;
        }

        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hades_ark() {
        let actual = ROUND_CONSTANTS[12];

        // https://github.com/starkware-industries/poseidon/blob/main/poseidon3.txt#L28
        let expected_num =
            "2404084503073127963385083467393598147276436640877011103379112521338973185443";
        let expected = Felt::from_dec_str(expected_num).unwrap();

        assert_eq!(actual, expected);
    }
}
