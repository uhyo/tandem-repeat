// "Ultra Naive" algorithm
use algorithm::result::AlgoResult;

pub fn algorithm(data: &[u8]) -> AlgoResult {
    let l = data.len();

    let mut max_from = 0;
    let mut max_length = 0;
    let mut max_count = 0;
    // 全ての開始地点iと全ての長さjについてアレする
    for i in 0..l {
        for j in 1..((l-i)/2+1) {
            // 位置iから長さjのパターンが何回繰り返されるか?
            let mut cnt = 0;
            let mut pos = i;
            'patt: while pos < l - j {
                // 1パターン試す
                for k in 0..j {
                    if data[i + k] != data[pos + k] {
                        // 繰り返せていないので終わり
                        break 'patt;
                    }
                }
                // 1回の繰り返しに成功
                cnt += 1;
                pos += j;
            }
            if 1 < cnt && (max_count as usize) * max_length < (cnt as usize) * j {
                // 最長記録更新
                max_from = i;
                max_length = j;
                max_count = cnt;
            }
        }
    }
    return AlgoResult {
        from: max_from,
        length: max_length,
        count: max_count,
    };
}

