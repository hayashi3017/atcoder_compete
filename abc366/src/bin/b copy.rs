use itertools::Itertools;
use proconio::{input, marker::Chars};

// https://atcoder.jp/contests/abc366/submissions/56530971
fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }
    let len = s.iter().map(|s| s.len()).max().unwrap();
    for s in s.iter_mut() {
        while s.len() < len {
            s.push('*');
        }
    }
    for i in 0..len {
        let mut res = vec![];
        let mut f = true;
        for j in 0..n {
            if s[j][i] != '*' || !f {
                f = false;
                res.push(s[j][i]);
            }
        }
        println!("{}", res.iter().rev().join(""));
    }
}
