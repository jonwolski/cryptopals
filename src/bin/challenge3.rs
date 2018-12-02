extern crate hex;
extern crate cryptopals;

use cryptopals::xor::*;

pub fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let input_bytes = hex::decode(input).unwrap();

    let output = brute_force_xor(&input_bytes);

    println!("{}", output);
}
