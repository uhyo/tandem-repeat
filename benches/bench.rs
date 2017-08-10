#![allow(dead_code)]
#[macro_use] extern crate bencher;
extern crate tandem_repeat;
extern crate rand;
use tandem_repeat::algorithm::{ultra_naive, divide, lcp};

use bencher::Bencher;
use rand::distributions::{IndependentSample, Range};

static CHARS: [u8; 4] = [0x41, 0x43, 0x47, 0x54];

fn bench500_lcp(b: &mut Bencher) {
    let range = Range::new(0, 4);
    b.iter(|| {
        let input = gen_input(&range, 500);
        lcp::algorithm(&input[..])
    });
}
fn bench10000_ultra_naive(b: &mut Bencher) {
    let range = Range::new(0, 4);
    b.iter(|| {
        let input = gen_input(&range, 10000);
        ultra_naive::algorithm(&input[..])
    });
}
fn bench10000_divide(b: &mut Bencher) {
    let range = Range::new(0, 4);
    b.iter(|| {
        let input = gen_input(&range, 10000);
        divide::algorithm(&input[..])
    });
}
fn bench10000_lcp(b: &mut Bencher) {
    let range = Range::new(0, 4);
    b.iter(|| {
        let input = gen_input(&range, 10000);
        lcp::algorithm(&input[..])
    });
}

fn gen_input(range: &Range<usize>, len: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        let r = range.ind_sample(&mut rng);
        result.push(CHARS[r]);
    }
    result.push(0);
    result
}

// benchmark_group!(benches, bench10000_ultra_naive, bench10000_divide, bench10000_lcp);
benchmark_group!(benches, bench500_lcp);
benchmark_main!(benches);
