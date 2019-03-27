#![feature(test)]

extern crate test;
use test::Bencher;

use payload::rand::Rand;

#[bench]
fn bench_rand(b: &mut Bencher) {
    let mut rand = Rand::new();

    b.iter(|| {
        (0..10000).fold(0, |old, _| old + rand.rand_u32())
    });
}

