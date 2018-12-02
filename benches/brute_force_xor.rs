#[macro_use]
extern crate criterion;
extern crate cryptopals;
extern crate hex;

use cryptopals::xor::*;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
	let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
	let input_bytes = hex::decode(input).unwrap();

	c.bench_function("brute-force xor", move |b| b.iter(|| brute_force_xor(&input_bytes)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
