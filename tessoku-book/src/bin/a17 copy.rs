#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/45335126
fn main() {
    input! {
        N: usize,
        A: [i64; N-1],
        B: [i64; N-2],
    }
    let mut dp = vec![0; N];
    dp[1] = A[0];
    for i in 2..N {
        dp[i] = std::cmp::min(dp[i - 1] + A[i - 1], dp[i - 2] + B[i - 2]);
    }

    let mut ans = vec![N];
    let mut i = N - 1;
    while 0 < i {
        if dp[i] - A[i - 1] == dp[i - 1] {
            ans.push(i);
            i -= 1;
        } else {
            ans.push(i - 1);
            i -= 2;
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().rev().join(" "));
}
