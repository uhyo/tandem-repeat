use algorithm::result::AlgoResult;
use algorithm::suffix_array::suffix_array;
use algorithm::lcp_array::lcp_array;

pub fn algorithm(data: &[u8]) -> AlgoResult {
    let sa = suffix_array(data);
    let lcp = lcp_array(data, &sa[..]);
    let len = data.len();

    // debug
    for i in 0..len {
        println!("[{:02}] {} {}", i, debug_suffix(data, sa[i]), lcp[i]);
    }

    


    AlgoResult {
        from: 0,
        length: 0,
        count: 0,
    }
}

// debug: 指定位置からのsuffixを表示
fn debug_suffix(data: &[u8], mut idx: usize) -> String {
    let mut result = String::new();
    let len = data.len();
    while idx < len {
        if data[idx] == 0 {
            result.push('$');
            break;
        }
        result.push(data[idx] as char);
        idx += 1;
    }
    result
}
