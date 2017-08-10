use algorithm::result::AlgoResult;
use algorithm::suffix_array::suffix_array;
use algorithm::lcp_array::{make_inv_sa, lcp_array};
use algorithm::rmq::rmq;

use std::mem;

// LCP Naive algorithm

pub fn algorithm(data_u8: &[u8]) -> AlgoResult {
    let len = data_u8.len();
    let mut data = Vec::with_capacity(len+1);
    for i in 0..len {
        data.push(data_u8[i] as u32);
    }
    // 終端を足す
    data.push(0);
    let len = len + 1;
    let sa = suffix_array(&data[..], 256);
    let inv_sa = make_inv_sa(&sa[..]);
    let lcp = lcp_array(&data[..], &sa[..], &inv_sa[..]);

    // debug
    /*
    for i in 0..len {
        println!("[{:02}] {} {}", i, debug_suffix(data, sa[i]), lcp[i]);
    }
    for i in 0..len {
        println!("[{:02}] -> [{:02}] {}", i, inv_sa[i], lcp[inv_sa[i]]);
    }
    */

    let r = rmq(&lcp[..]);

    let mut max_from = 0;
    let mut max_length = 0;
    let mut max_count = 0;
    // 全ての開始地点iと全ての長さjについてアレする
    for i in 0..len {
        for j in 1..((len-i)/2+1) {
            // 位置iから長さjのパターンが何回繰り返されるか?
            let mut idx = inv_sa[i];
            let mut idx2= inv_sa[i+j];
            if idx > idx2 {
                mem::swap(&mut idx, &mut idx2);
            }
            // この間の最小のLCPが全体のlcpになる
            let icpxy = r.query(idx, idx2 - 1);
            let cnt = lcp[icpxy] / j + 1;
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
#[allow(dead_code)]
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
