use proconio::{input, marker::Chars};
use superslice::Ext;

// https://atcoder.jp/contests/abc363/submissions/55822164
fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars,
    }
    let mut ans = 0;
    s.sort();
    while {
        let mut flag = true;
        for i in 0..n - k + 1 {
            if s[i..i + k].iter().collect::<String>() == s[i..i + k].iter().rev().collect::<String>() {
                flag = false;
            }
        }
        if flag {
            ans += 1;
        }
        s.next_permutation()
    } {}
    println!("{}", ans);
}
