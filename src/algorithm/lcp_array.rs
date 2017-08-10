// LCP arrayを作成

pub fn lcp_array(data: &[u32], sa: &[usize], inv_sa: &[usize]) -> Vec<usize> {
    let len = sa.len();
    // 高さ配列
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(0);
    }

    let mut mem = 0;
    for i in 0..len {
        // 次の接尾辞とのLCP length
        let idx = inv_sa[i];    //長い方からi番目の接尾辞はsa[idx]にある
        if idx == len-1 {
            // 一番最後なのでない
            continue;
        }
        let j = sa[idx+1];
        // i番目とj番目から比較する
        let mut cnt = 0;
        if mem > 0 {
            // 前回の結果を用いてスキップできる
            cnt = mem-1;
        }
        while i+cnt < len && j+cnt < len {
            if data[i+cnt] != data[j+cnt] {
                // ここまで
                break;
            }
            cnt += 1;
        }
        // LCPの長さはcnt
        result[idx] = cnt;
        mem = cnt;
    }
    result
}

// 逆接尾辞配列を作成
pub fn make_inv_sa(sa: &[usize]) -> Vec<usize> {
    let len = sa.len();
    let mut inv_sa = Vec::with_capacity(len);
    for _ in 0..len {
        inv_sa.push(0);
    }
    for i in 0..len {
        inv_sa[sa[i]] = i;
    }
    inv_sa
}
