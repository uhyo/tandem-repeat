use getopts::Options;

use std::env;

#[derive(Debug, PartialEq, Eq)]
pub enum Algorithm {
    UltraNaive, // 1
    Divide,     // 2
}
#[derive(Debug)]
pub struct Opts {
    pub algorithm: Algorithm,
}

pub fn getopts() -> Opts {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("a", "algorithm", "Algorithm", "NAME");

    let opts =
        match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(err) => {
                panic!(err.to_string());
            },
        };

    let a = opts.opt_str("algorithm").unwrap_or(String::from("ultranaive"));
    let algorithm = 
        if a == "ultranaive" || a == "1" {
            Algorithm::UltraNaive
        } else if a == "divide" || a == "2" {
            Algorithm::Divide
        } else {
            panic!("Unknown algorithm {}", a)
        };

    return Opts {
        algorithm,
    };
}
