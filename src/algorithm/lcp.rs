use algorithm::result::AlgoResult;
use algorithm::suffix_array::suffix_array;
use algorithm::lcp_array::{make_inv_sa, lcp_array};

// LCP Naive algorithm

pub fn algorithm(data: &[u8]) -> AlgoResult {
    let sa = suffix_array(data);
    let inv_sa = make_inv_sa(&sa[..]);
    let lcp = lcp_array(data, &sa[..], &inv_sa[..]);
    let len = data.len();

    // debug
    for i in 0..len {
        println!("[{:02}] {} {}", i, debug_suffix(data, sa[i]), lcp[i]);
    }

    let mut max_from = 0;
    let mut max_length = 0;
    let mut max_count = 0;
    // 全ての開始地点iと全ての長さjについてアレする
    for i in 0..len {
        for j in 1..((len-i)/2+1) {
            // 位置iから長さjのパターンが何回繰り返されるか?
            let idx = inv_sa[i];
            let idx2= inv_sa[i+j];
            let cnt =
                if idx < idx2 {
                    // i+jのほうが後ろにある
                    // i  : pp...ppq
                    // ...
                    // i+j: pp...pq
                    lcp[idx] / j + 1
                } else {
                    // i+j: pp...pq
                    // i  : pp...ppq
                    lcp[idx2] / j + 1
                };
            if 1 < cnt && max_count * max_length < cnt * j {
                // 最長記録更新
                max_from = i;
                max_length = j;
                max_count = cnt;
            }
        }
    }

    AlgoResult {
        from: max_from,
        length: max_length,
        count: max_count as u32,
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
