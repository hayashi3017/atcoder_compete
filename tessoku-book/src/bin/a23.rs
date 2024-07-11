use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize;n];m]
    }

    let mut dp = vec![vec![1_000_000_000usize; n]; m];
    dp[0][0] = 0;
    for i in 1..=m {
        for j in 0..n {
            // tips: 2進数へ変換する、任意の数値で0埋めする
            let b = format!("{:0n$b}", j, n = n);

            // tips: 2進数文字列から10進数へ変換する
            // let binary = a[i][j].to_string();
            // let _ = usize::from_str_radix(&binary, 2);

            dp[i][j] = dp[i][j].min(dp[i-1][j]);
            
        }
    }
}
