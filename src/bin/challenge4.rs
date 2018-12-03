extern crate cryptopals;
extern crate hex;

use cryptopals::xor::*;
use std::io;
use std::io::prelude::*;

pub fn main() {
    let stdin = io::stdin();
    let results: Vec<ComputedXor> = stdin
        .lock()
        .lines()
        .map(|line| brute_force_xor(&hex::decode(line.unwrap()).unwrap()))
        .flatten()
        //TODO change this to a score normalized to the size of the string
        .filter(|computed_xor| computed_xor.score >= 8_000_000_000)
        .filter(|computed_xor| computed_xor.score / (computed_xor.value.len() as u64) >= 250_000_000)
        .collect();
    for res in results {
        println!("{}", res);
    }
}
