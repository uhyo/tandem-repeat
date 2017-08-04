// Induced Sorting [Nong-Zhang-Chan '09]
use std::cmp::Ordering;

pub struct SuffixArray{
}

pub fn suffix_array(data: &[u8]) -> SuffixArray {
    let len = data.len();
    // < and >
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
    println!("{:?}", types);
    SuffixArray {
    }
}
