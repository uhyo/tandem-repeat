extern crate tandem_repeat;
use tandem_repeat::options::{getopts, Algorithm};
use tandem_repeat::algorithm;
use std::io;
use std::io::Read;

fn main() {
    // read options
    let opts = getopts();
    // read standard input
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();

    let result =
        match opts.algorithm {
            Algorithm::UltraNaive => algorithm::ultra_naive::algorithm(buf),
        };
    let result = result.unwrap();

    println!("result: {:?}", result);
}
