extern crate hex;
extern crate cryptopals;

use cryptopals::xor::*;
use std::io;
use std::io::prelude::*;

pub fn main() {
	let stdin = io::stdin();
	let results : Vec<ComputedXor> =
		stdin.lock().lines().map( |line| {
			brute_force_xor(&hex::decode(line.unwrap()).unwrap())
		})
		.filter(|computed_xor| computed_xor.score >= 700_000_000)
		.collect();
	for res in results {
		println!("{}", res);
	};
}

