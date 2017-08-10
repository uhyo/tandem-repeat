#![allow(dead_code)]
#[macro_use] extern crate bencher;
extern crate tandem_repeat;
extern crate rand;
use tandem_repeat::algorithm::{ultra_naive, divide, lcp, lcp_divide};

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
fn bench1_ultra_naive(b: &mut Bencher) {
    let range = Range::new(0, 4);
    b.iter(|| {
        let input = gen_input(&range, 1000);
        ultra_naive::algorithm(&input[..])
    });
}
fn bench1_divide(b: &mut Bencher) {
    let range = Range::new(0, 4);
    b.iter(|| {
        let input = gen_input(&range, 1000);
        divide::algorithm(&input[..])
    });
}
fn bench1_lcp(b: &mut Bencher) {
    let range = Range::new(0, 4);
    b.iter(|| {
        let input = gen_input(&range, 1000);
        lcp::algorithm(&input[..])
    });
}
fn bench1_lcp_divide(b: &mut Bencher) {
    let range = Range::new(0, 4);
    b.iter(|| {
        let input = gen_input(&range, 1000);
        lcp_divide::algorithm(&input[..])
    });
}

fn gen_input(range: &Range<usize>, len: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        let r = range.ind_sample(&mut rng);
        result.push(CHARS[r]);
    }
    result
}

benchmark_group!(benches, bench1_ultra_naive, bench1_divide, bench1_lcp, bench1_lcp_divide);
// benchmark_group!(benches, bench500_lcp);
benchmark_main!(benches);
