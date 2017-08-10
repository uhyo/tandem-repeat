// LCP + 分割統治法
use algorithm::result::AlgoResult;
use algorithm::suffix_array::suffix_array;
use algorithm::lcp_array::{make_inv_sa, lcp_array};
use algorithm::rmq::{rmq, Rmq};

use std::mem;

pub fn algorithm(data_u8: &[u8]) -> AlgoResult {
    let len = data_u8.len();
    // データをu32に（順方向と逆方向）
    let mut data = Vec::with_capacity(len+1);
    let mut data_rev = Vec::with_capacity(len+1);
    for i in 0..len {
        data.push(data_u8[i] as u32);
        data_rev.push(data_u8[i] as u32);
    }
    data_rev.reverse();
    // 終端を足す
    data.push(0);
    data_rev.push(0);
    // println!("{:?}", data_rev);
    let len = len + 1;  //終端も入れたやつ

    let sa = suffix_array(&data[..], 256);
    let sa_rev = suffix_array(&data_rev[..], 256);

    let inv_sa = make_inv_sa(&sa[..]);
    let inv_sa_rev = make_inv_sa(&sa_rev[..]);

    let lcp = lcp_array(&data[..], &sa[..], &inv_sa[..]);
    let lcp_rev = lcp_array(&data_rev[..], &sa_rev[..], &inv_sa_rev[..]);

    let r = rmq(&lcp[..]);
    let r_rev = rmq(&lcp_rev[..]);

    for i in 0..len {
        // println!("[{:02}] {} {}", i, debug_suffix(&data_rev[..], sa_rev[i]), lcp_rev[i]);
    }

    return rec(data.len(), &lcp[..], &inv_sa[..], &r, &lcp_rev[..], &inv_sa_rev[..], &r_rev, 0, len, 0);
}

fn rec(l: usize, lcp: &[usize], inv_sa: &[usize], r: &Rmq, lcp_rev: &[usize], inv_sa_rev: &[usize], r_rev: &Rmq, start: usize, end: usize, mut alpha: usize) -> AlgoResult {
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
    // println!("start = {}, end = {}, point = {}", start, end, point);

    // pointを含む繰り返しを探す
    let mut max_count = 0;
    let mut max_length = 0;
    let mut max_from = 0;
    for j in 1..(end - point + 2) {
        // 長さjのパターン

        // まず右方向へ繰り返しを探す
        if point + j < end {
            let mut idx = inv_sa[point];
            let mut idx2= inv_sa[point+j];
            if idx > idx2 {
                mem::swap(&mut idx, &mut idx2);
            }
            // 位置iからj文字のパターンが繰り返している長さ
            let icpxy = r.query(idx, idx2 - 1);
            // println!("j={} right1: {} ({})", j, icpxy, lcp[icpxy]);
            if lcp[icpxy] > 0 {
                // 繰り返しがあった

                // ありえる繰り返しの右端
                let i2 = point + lcp[icpxy] + j - 1;
                if i2 < end {
                    // はみ出たらここでは扱わない
                    // 座標を逆にする
                    let i2 = l - 2 - i2;
                    let i3 = i2 + j;
                    let mut newidx = inv_sa_rev[i2];
                    let mut newidx2 = inv_sa_rev[i3];
                    if newidx > newidx2 {
                        mem::swap(&mut newidx, &mut newidx2);
                    }
                    let icpxy2 = r_rev.query(newidx, newidx2 - 1);
                    let ln = lcp_rev[icpxy2];
                    // println!("left1: {}-{} {}-{} : {} ({})", i2, i3, newidx, newidx2, icpxy2, ln);
                    // i3から（左へ）lnの長さだけ繰り返しがある
                    let cnt = ln / j + 1;
                    if cnt > 1 {
                        // println!("FOUND {}-{}-{}", l - 2 - (i2 + cnt * j - 1), j, cnt);
                    }
                    if 1 < cnt && max_count * max_length < cnt * j {
                        // 最長記録
                        // 座標を戻す
                        max_from = l - 2 - (i2 + cnt * j - 1);
                        max_length = j;
                        max_count = cnt;
                    }
                }
            }
        }
        // 左からも探す
        let i2 = l - 2 - point;
        if point >= start + j {
            let i3 = i2 + j;
            let mut idx = inv_sa_rev[i2];
            let mut idx2= inv_sa_rev[i3];
            if idx > idx2 {
                mem::swap(&mut idx, &mut idx2);
            }
            let icpxy = r_rev.query(idx, idx2 - 1);
            // println!("j={} left2: {}-{} {}-{} {} ({})", j, i2, i3, idx, idx2, icpxy, lcp_rev[icpxy]);
            if lcp_rev[icpxy] > 0 {
                // 左へ繰り返しがあった
                let e = i2 + j + lcp_rev[icpxy] - 1;
                if l - 2 < e {
                    // overflow
                    continue;
                }
                // 座標を戻す
                let e = l - 2 - e;
                if e < start {
                    // 戻りすぎ
                    continue;
                }
                // 再度右へ
                let end2 = e + j;
                let mut idx = inv_sa[e];
                let mut idx2= inv_sa[end2];
                if idx > idx2 {
                    mem::swap(&mut idx, &mut idx2);
                }
                let lcpxy = r.query(idx, idx2 - 1);
                let ln = lcp[lcpxy];
                let cnt = ln / j + 1;
                if 1 < cnt && max_count * max_length < cnt * j {
                    // 最長記録
                    max_from = e;
                    max_length = j;
                    max_count = cnt;
                }
            }
        }
    }
    if alpha < max_count * max_length {
        alpha = max_count * max_length;
    }
    // 分割したほうを調べる
    let left = rec(l, lcp, inv_sa, r, lcp_rev, inv_sa_rev, r_rev, start, point, alpha);
    if alpha < (left.count as usize) * left.length {
        max_count = left.count as usize;
        max_from = left.from;
        max_length = left.length;
        alpha = max_count * max_length;
    }
    let right = rec(l, lcp, inv_sa, r, lcp_rev, inv_sa_rev, r_rev, point + 1, end, alpha);
    if alpha < (right.count as usize) * right.length {
        max_count = right.count as usize;
        max_from = right.from;
        max_length = right.length;
    }
    return AlgoResult {
        from: max_from,
        length: max_length,
        count: max_count as u32,
    };
}

#[allow(dead_code)]
fn debug_suffix(data: &[u32], mut idx: usize) -> String {
    let mut result = String::new();
    let len = data.len();
    while idx < len {
        if data[idx] == 0 {
            result.push('$');
            break;
        }
        result.push((data[idx] as u8) as char);
        idx += 1;
    }
    result
}
