extern crate tandem_repeat;
use tandem_repeat::options::{getopts, Algorithm};
use tandem_repeat::algorithm;
use std::io;
use std::io::Read;
use std::str::from_utf8;

fn main() {
    // read options
    let opts = getopts();
    // read standard input
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();
    // 改行は邪魔なので消す
    buf.retain(|&c| c != 0x0A && c != 0x0D);

    let result =
        match opts.algorithm {
            Algorithm::UltraNaive => algorithm::ultra_naive::algorithm(&buf[..]),
            Algorithm::Divide => algorithm::divide::algorithm(&buf[..]),
            Algorithm::Lcp => algorithm::lcp::algorithm(&buf[..]),
            Algorithm::LcpDivide => panic!("Not implemented"),
        };

    println!("result: {:?}", result);
    let patt = &buf[result.from .. (result.from+result.length)];
    let patt = from_utf8(patt).unwrap();
    println!("{} x {}", patt, result.count);
}
