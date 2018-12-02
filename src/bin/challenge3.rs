extern crate cryptopals;
extern crate hex;

use cryptopals::*;

pub fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let input_bytes = hex::decode(input).unwrap();

    let mut high_score = 0.0;
    let mut out_string = Vec::new();
    let mut best_mask = 0;

    for mask in 0..=255 {
        let mut score = 0.0;
        let out: Vec<u8> = input_bytes
            .iter()
            .map(|b| {
                let unmasked = b ^ mask;
                score += LETTER_FREQUENCIES.get(&(unmasked as char)).unwrap_or(&0.0);
                unmasked
            }).collect();

        if score > high_score {
            high_score = score;
            out_string = out;
            best_mask = mask;
        }
    }

    println!(
        "mask: {}, val: {}",
        best_mask,
        String::from_utf8_lossy(&out_string)
    );
}
