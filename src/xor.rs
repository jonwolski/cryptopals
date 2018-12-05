use super::*;
use rayon::prelude::*;
use std::fmt;

const SCORE_THRESHOLD: f32 = 0.265;

#[derive(PartialEq, Debug)]
pub struct ComputedXor {
    pub mask: u8,
    pub score: f32,
    pub value: Vec<u8>,
}

impl Eq for ComputedXor {}

use std::cmp::Ordering;
impl Ord for ComputedXor {
    fn cmp(&self, other: &ComputedXor) -> Ordering {
        // TODO: must ensure that neither is ever NaN
        if self.score > other.score {
            Ordering::Greater
        } else if self.score < other.score {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for ComputedXor {
    fn partial_cmp(&self, other: &ComputedXor) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for ComputedXor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "score: {:000.3}, mask: {}, val: {}",
            self.score,
            self.mask,
            String::from_utf8_lossy(&self.value)
        )
    }
}

pub fn brute_force_xor(input: &[u8]) -> Vec<ComputedXor> {
    // rayon par_iter is not implemented for InclusiveRange
    let mut results = (0..0x100)
        .into_par_iter()
        .map(|mask_int| {
            let mask = mask_int as u8;
            let mut score = 0.0_f32;
            let value = input
                .iter()
                .map(|b| {
                    let unmasked = b ^ mask;
                    score += LETTER_FREQUENCIES.get(&unmasked).unwrap_or(&0.0);
                    unmasked
                }).collect::<Vec<u8>>();
            ComputedXor {
                score: score / (value.len() as f32),
                value: value,
                mask: mask,
            }
        }).filter(|c| c.score > SCORE_THRESHOLD)
        .collect::<Vec<ComputedXor>>();
    results.sort();
    results.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 3
    #[test]
    fn brute_force_xor_test() {
        let input =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();
        let results = brute_force_xor(&input);
        let maybe_output = results.first();
        let output = maybe_output.unwrap();
        assert_eq!(88, output.mask);
        assert_eq!(
            "Cooking MC's like a pound of bacon",
            String::from_utf8_lossy(&output.value)
        );
    }
}
