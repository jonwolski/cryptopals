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
        .collect();
    for res in results {
        println!("{}", res);
    }
}
