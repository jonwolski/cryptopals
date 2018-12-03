use super::*;
use rayon::prelude::*;
use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct ComputedXor {
    pub mask: u8,
    pub score: u64,
    pub value: Vec<u8>,
}

use std::cmp::Ordering;
impl Ord for ComputedXor {
    fn cmp(&self, other: &ComputedXor) -> Ordering {
		self.score.cmp(&other.score)
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

pub fn brute_force_xor(input: &[u8]) -> ComputedXor {
    // rayon par_iter is not implemented for InclusiveRange
    (0..0x100)
        .into_par_iter()
        .map(|mask_int| {
            let mask = mask_int as u8;
            let mut score = 0;
            let value = input
                .iter()
                .map(|b| {
                    let unmasked = b ^ mask;
                    score += LETTER_FREQUENCIES.get(&unmasked).unwrap_or(&0);
                    unmasked
                }).collect();
            ComputedXor {
                score: score,
                value: value,
                mask: mask,
            }
        }).max().expect("Iterator cannot be empty, because it is based on statically defined range.")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 3
    #[test]
    fn brute_force_xor_test() {
        let input = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
        let output = brute_force_xor(&input);
        assert_eq!( 88, output.mask);
        assert_eq!( "Cooking MC's like a pound of bacon", String::from_utf8_lossy(&output.value));
    }
}
