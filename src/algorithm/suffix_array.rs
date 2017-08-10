// Induced Sorting [Nong-Zhang-Chan '09]
// 参考: http://qiita.com/tobi-c/items/cf450a7b1d6b59f332d1
//       http://sile.hatenablog.jp/entry/20101213/1292190698
use std::cmp::Ordering;

pub fn suffix_array(data: &[u32], max_alphabet: usize) -> Vec<usize> {
    let len = data.len();
    // < (S-Type) and > (L-Type)
    let mut types = Vec::with_capacity(len);
    // 一番うしろはよくわからない
    types.push(false);
    for i in 1..len {
        // 一番後ろのひとつ前から見ていく
        let idx = len - i - 1;
        let cur = data[idx];
        let next= data[idx+1];
        let ord =
            match cur.cmp(&next) {
                Ordering::Less => {
                    // idxは"<"タイプだ
                    false
                },
                Ordering::Greater => {
                    // idxは">"タイプだ
                    true
                },
                Ordering::Equal => {
                    // 直前と同じだ
                    types[i-1]
                },
            };
        types.push(ord);
    }
    types.reverse();
    // LMSを探す
    let mut lms = Vec::with_capacity(len);
    let mut lms_sa = Vec::new();    // LMSの位置を抜き出した配列
    let mut flg = false; //先頭はLMSにならない?
    for i in 0..len {
        if !types[i] {
            // >
            lms.push(flg);
            if flg {
                lms_sa.push(i);
            }
            flg = false;
        } else {
            lms.push(false);
            flg = true;
        }
    }
    // 1-1 -- 1-3
    let res = induced_sort(&types[..], data, &lms_sa[..], max_alphabet);
    // 1-4
    let mut lms_sa = Vec::new();
    for i in 0..len {
        let idx = res[i];
        if lms[idx] {
            // LMSのみ抽出
            lms_sa.push(idx);
        }
    }

    // 2-1
    ///// 出現位置順に並べたいのでそれをアレする
    let mut names = Vec::with_capacity(len);
    for _ in 0..len {
        names.push(None);
    }
    ///// 番号をつける
    let salen = lms_sa.len();
    let mut name: u32 = 0;
    let mut dup = false;
    for i in 0..salen {
        if i != 0 && !lms_eq(data, &lms[..], lms_sa[i-1], lms_sa[i]) {
            // lms_eqは普通に比較してもO(n)のはず
            name += 1;
        } else if i != 0 {
            // 重複あった
            dup = true;
        }
        names[lms_sa[i]] = Some(name);
    }
    ///// LMSたちのsuffix arrayを作成
    let lms_sa =
        if dup {
            // 再帰
            let mut new_num = Vec::with_capacity(salen);
            let mut num_inv = Vec::with_capacity(salen);
            for i in 0..len {
                if let Some(n) = names[i] {
                    new_num.push(n);
                    num_inv.push(i);
                }
            }
            // 2-2
            let mut lms_sa = Vec::with_capacity(salen);
            // 再帰
            let sub = suffix_array(&new_num[..], (name as usize) + 1);
            // subをこの文字列の位置に直す
            for i in 0..salen {
                lms_sa.push(num_inv[sub[i]]);
            }
            lms_sa
        } else {
            lms_sa
        };
    // 3-1 -- 3-3
    let result = induced_sort(&types[..], data, &lms_sa[..], max_alphabet);

    result
}

// bucket sort
fn make_bucket(data: &[u32], max_alphabet: usize) -> Vec<usize> {
    let len = data.len();
    let mut bucket = Vec::with_capacity(max_alphabet);
    for _ in 0..max_alphabet {
        bucket.push(0);
    }
    for i in 0..len {
        bucket[data[i] as usize] += 1;
    }
    bucket
}
// 先頭からの位置（右詰め用）に直す
fn from_right_bucket(bucket: &mut Vec<usize>) {
    let mut sum = 0;
    let len = bucket.len();
    for i in 0..len {
        // sum: この文字が始まる位置
        sum += bucket[i];
        bucket[i] = sum;   // bucketは終わりの位置に直す
    }
}

// induced sort
fn induced_sort(types: &[bool], data: &[u32], sa: &[usize], max_alphabet: usize) -> Vec<usize> {
    let len = data.len();
    let salen = sa.len();
    // 結果を入れるやつ
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(None);
    }
    // 各文字を数えたもの
    let default_bucket = make_bucket(data, max_alphabet);
    // 1-1 / 3-1
    let mut bucket = default_bucket.clone();
    from_right_bucket(&mut bucket);
    ///// SAを後ろから入れていく
    for i in 0..salen {
        let i = salen - 1 - i;
        let k = sa[i];
        let ch = data[k] as usize;
        bucket[ch] -= 1;
        result[bucket[ch]] = Some(k);
    }
    // 1-2 / 3-2
    let mut bucket = default_bucket.clone();
    ///// 先頭からの位置（ただし左から）
    let mut sum = 0;
    for i in 0..max_alphabet {
        let s = sum;
        sum += bucket[i];
        bucket[i] = s;
    }
    ///// 入れていく
    for i in 0..len {
        if let Some(idx) = result[i] {
            if idx > 0 && types[idx-1] {
                // L-type
                let lidx = idx-1;
                let ch = data[lidx] as usize;
                result[bucket[ch]] = Some(lidx);
                bucket[ch] += 1;
            }
        }
    }
    // 1-3 / 3-3
    let mut bucket = default_bucket.clone();
    from_right_bucket(&mut bucket);
    for i in 0..len {
        // 後ろから
        let i = len - 1 - i;
        if let Some(idx) = result[i] {
            if idx > 0 && !types[idx-1] {
                // 前がS-type
                let sidx = idx-1;
                let ch = data[sidx] as usize;
                bucket[ch] -= 1;
                result[bucket[ch]] = Some(sidx);
            }
        }
    }
    // Someを外す
    let mut result2 = Vec::with_capacity(len);
    for i in 0..len {
        result2.push(result[i].unwrap());
    }
    result2
}

// LMS部分文字列が一致するかどうかの比較
fn lms_eq(data: &[u32], lms: &[bool], i: usize, j: usize) -> bool {
    let len = data.len();
    if data[i] != data[j] {
        return false;
    }
    let mut idxi = i + 1;
    let mut idxj = j + 1;
    while idxi < len && idxj < len {
        if data[idxi] != data[idxj] {
            return false;
        }
        if lms[idxi] && lms[idxj] {
            // 次のLMSまで同じだった
            return true;
        }
        idxi += 1;
        idxj += 1;
    }
    // null文字用
    return idxi == idxj;
}
