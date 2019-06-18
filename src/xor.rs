use super::*;
use itertools::Itertools;
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

fn guess_key_size(input: &[u8]) -> Option<usize> {
    const MAX_KEY_SIZE : usize = 40;
    (2..= MAX_KEY_SIZE).filter_map(|keysize| {
        let mut iter = input.chunks(keysize);

        let s1 = iter.next();
        let s2 = iter.next();
        let s3 = iter.next();

        match (s1, s2, s3) {
            /* Max keysize is 40 bytes, 320 bits, but hamming returns a u64.
             * Shift the value 2.5 bytes so thqt we can do integer arithmetic on the division
             * but still have enough precisions (after rounding) for the `min` to be meaningful
             */
            (Some(s1), Some(s2), Some(s3)) if s1.len() == s3.len() => {
                Some(((
                            hamming::distance(&s1, &s2) +
                            hamming::distance(&s1, &s3) +
                            hamming::distance(&s2, &s3) ) *0x1_0000/(keysize as u64), keysize))
            },
            _ => None
        }
    })
    .sorted().map(|m| {println!("{:?}", m); m})
    .min()
    .map(|r| r.1)
}


pub fn brute_force_xor_repeating<'a>(input: &[u8]) -> Vec<ComputedXor> {
    let best_key_size = guess_key_size(input);
    if let Some(keysize) = best_key_size {
        println!("keysize: {:?}", keysize);
        let mut stripes = vec![Vec::with_capacity(input.len()/keysize + 1); keysize];
        //println!("{:?}", stripes);
        input.iter().enumerate().for_each(|(i, c)| {
            stripes[i % keysize].push(*c)
        //    ;println!("{:?}", stripes)
        });
        println!("{:?}", stripes);
        stripes.iter().filter_map(|s| {
        //    println!("{:?}", s);
            let mut results = brute_force_xor(s);
        //    println!("brute_force_xor({:?}): {:?}", s, results);
            results.pop()
        }).collect::<Vec<_>>()
    } else {
        Vec::new()
    }
}

pub fn brute_force_xor(input: &[u8]) -> Vec<ComputedXor> {
    let mut results = (0u8..=0xFF)
        .into_par_iter()
        .map(|mask| {
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

pub fn xor_repeated_key(key: &[u8], message: &[u8]) -> Vec<u8> {
    key.into_iter()
        .cycle()
        .zip(message.into_iter())
        .map(|(k, v)| v ^ k)
        .collect()
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

    // Exercise 5
    #[test]
    fn xor_repeated_key_test() {
        assert_eq!("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f", hex::encode(&xor_repeated_key(b"ICE", b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal")));
    }

    // Exercise 5
    #[test]
    fn xor_repeated_key_longer_key_test() {
        assert_eq!("0b16172d1b0b064d7404034857000548300c1063130c0f4a27411f111e0a0848280d01631c0c0c0f3f04642d570e0c482a1104390b451605360f4e2d570106093b430463111c0c0f320d", hex::encode(&xor_repeated_key(b"IceCreamSandwich", b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal")));
    }

    // Exercise 6
    #[test]
    #[allow(dead_code)]
    fn brute_force_xor_repeating_test() {
        let corpus = "But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure?";
        let key = "IceCreamSandwich";
        let cipher_text = hex::encode(&xor_repeated_key(key.as_bytes(), corpus.as_bytes()));
        let input = hex::decode(cipher_text).unwrap();
        let results = brute_force_xor_repeating(&input);
        let maybe_output = results.first();
        let output = maybe_output.unwrap();
        assert_eq!(
            corpus,
            String::from_utf8_lossy(&output.value)
        );
    }

    #[test]
    fn guess_key_size_test() {
        let input =
            hex::decode("0b16172d1b0b064d7404034857000548300c1063130c0f4a27411f111e0a0848280d01631c0c0c0f3f04642d570e0c482a1104390b451605360f4e2d570106093b430463111c0c0f320d")
            .unwrap();
        let actual = guess_key_size(&input);
        assert_eq!(Some("IceCreamSandwich".len()), actual);
    }
}
