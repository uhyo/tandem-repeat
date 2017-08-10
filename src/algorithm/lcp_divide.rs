// LCP + 分割統治法
use algorithm::result::AlgoResult;
use algorithm::suffix_array::suffix_array;
use algorithm::lcp_array::{make_inv_sa, lcp_array};
use algorithm::rmq::{rmq, Rmq};

use std::mem;

pub fn algorithm(data: &[u8]) -> AlgoResult {
    let sa = suffix_array(data);
    let inv_sa = make_inv_sa(&sa[..]);
    let lcp = lcp_array(data, &sa[..], &inv_sa[..]);
    let len = data.len();

    let r = rmq(&lcp[..]);

    return rec(data, &lcp[..], &rmq, 0, l, 0);
}

fn rec(data: &[u8], lcp: &[usize], rmq: &Rmq, start: usize, end: usize, alpha: usize) -> AlgoResult {
    // 区間の長さ
    if end <= start + 1 {
        // もう区間がない
        return AlgoResult {
            from: 0,
            length: 0,
            count: 0,
        };
    }
    let len = end - start;
    if len <= alpha {
        // すでに長さalphaの繰り返しが見つかっているのでこれは必要ない
        return AlgoResult {
            from: 0,
            length: 0,
            count: 0,
        };
    }
    // 中間点
    let point = (start + end) / 2;

    // pointを含む繰り返しを探す
    let mut max_count = 0;
    let mut max_len = 0;
    let mut max_from = 0;
    for j in 1..(len+1) {
        // 長さjのパターン
        // まず左へ
        let mut cnt = 0;
        let mut pos = point;
        let mut point2 = point;
        let mut cur_from = 0;
        'patt: while start <= pos {
            for k in 0..j {
                if data[point - k] != data[pos] {
                    // 違うじゃん
                    // 新しい起点を設定
                    cur_from = pos + 1;
                    point2 = cur_from;
                    pos = point + 1 - k;
                    break 'patt;
                }
                if pos == start {
                    // 最後まで到達した
                    point2 = point - k;
                    cur_from = 0;
                    pos = point2;
                    if k == 0 {
                        cnt -= 1;
                    }
                    break 'patt;
                }
                pos -= 1;
            }
            // 1回の繰り返しに成功
            cnt += 1;
        }
        // 右へ
        'pattr: while pos + j < end {
            for k in 0..j {
                if data[point2 + k] != data[pos + k] {
                    break 'pattr;
                }
            }
            cnt += 1;
            pos += j;
        }
        // 結果
        if 1 < cnt && (max_count as usize) * max_len < (cnt as usize) * j {
            max_from = cur_from;
            max_count = cnt;
            max_len = j;
        }
    }
    if alpha < (max_count as usize) * max_len {
        alpha = (max_count as usize) * max_len;
    }
    // 分割したほうを調べる
    let left = rec(data, start, point, alpha);
    if alpha < (left.count as usize) * left.length {
        max_count = left.count;
        max_from = left.from;
        max_len = left.length;
        alpha = (max_count as usize) * max_len;
    }
    let right = rec(data, point + 1, end, alpha);
    if alpha < (right.count as usize) * right.length {
        max_count = right.count;
        max_from = right.from;
        max_len = right.length;
    }
    return AlgoResult {
        from: max_from,
        length: max_len,
        count: max_count,
    };
}
