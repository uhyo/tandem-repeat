use algorithm::result::AlgoResult;
use algorithm::suffix_array::suffix_array;

pub fn algorithm(data: &[u8]) -> AlgoResult {
    let _ = suffix_array(data);

    AlgoResult {
        from: 0,
        length: 0,
        count: 0,
    }
}
