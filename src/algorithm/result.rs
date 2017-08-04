#[derive(Debug,PartialEq,Eq)]
pub struct AlgoResult {
    // 繰り返しの開始地点
    pub from: usize,
    // パターンの長さ
    pub length: usize,
    // 繰り返し回数
    pub count: u32,
}
