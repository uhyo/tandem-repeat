// RMQ
// http://qiita.com/okateim/items/e2f4a734db4e5f90e410
// ただし前処理O(nlogn)のSparse Tree Algorithm

use std::cmp::min;

pub struct Rmq<'a> {
    lcp: &'a [usize],
    st: Vec<Vec<usize>>,
}
impl<'a> Rmq<'a> {
    pub fn query(&'a self, x: usize, y: usize) -> usize {
        // println!("query {}, {}.", x, y);
        if x == y {
            return x;
        }
        let k = ((y - x) as f64).log2() as usize;
        // println!("k = {}.", k);
        let yi = y - 2usize.pow(k as u32) + 1;
        // println!("xran = [{}, {}].", x, x + 2usize.pow(k as u32) -1);
        // println!("yran = [{}, {}].", yi, yi + 2usize.pow(k as u32) -1);
        let i = self.st[k][x];
        let j = self.st[k][yi]; 
        // println!("xi = {} ({}), yi = {} ({}).", i, self.lcp[i], j, self.lcp[j]);
        if self.lcp[i] < self.lcp[j] {
            i
        } else {
            j
        }
    }
}

pub fn rmq<'a>(lcp: &'a [usize]) -> Rmq<'a> {
    let len = lcp.len();
    // ブロックの数
    let m = ((len as f64).log2() + 1.0) as usize;

    let mut st: Vec<Vec<usize>> = Vec::new();
    for p in 0..m {
        let mut res = Vec::with_capacity(len);
        if p == 0 {
            // DPのbase case
            for i in 0..len {
                res.push(i);
            }
        } else {
            // println!("p = {}", p);
            for i in 0..len {
                let j = min(len-1, i + 2usize.pow((p-1) as u32));
                let x = st[p-1][i];
                let y = st[p-1][j];
                // println!("i = {}, j = {}. x = {} ({}), y = {} ({}), size = {}", i, j, x, lcp[x], y, lcp[y], 2usize.pow((p-1) as u32));
                if lcp[x] < lcp[y] {
                    res.push(x);
                } else {
                    res.push(y);
                }
            }
        }
        st.push(res);
    }

    Rmq {
        st,
        lcp,
    }
}
