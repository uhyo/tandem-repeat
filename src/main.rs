extern crate tandem_repeat;
use tandem_repeat::options::getopts;

fn main() {
    let opts = getopts();

    println!("Hello, world! {:?}", opts);
}
