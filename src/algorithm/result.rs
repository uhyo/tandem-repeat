#[derive(Debug)]
pub struct AlgoResult {
    // 繰り返しの開始地点
    from: usize,
    // パターンの長さ
    length: usize,
    // 繰り返し回数
    count: usize,
}
