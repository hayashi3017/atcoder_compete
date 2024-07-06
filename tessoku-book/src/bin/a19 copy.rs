use proconio::input;
// https://atcoder.jp/contests/tessoku-book/submissions/41512668
fn main() {
    input! {
        n:usize,w:usize,wv:[(usize,usize);n]
    }
    // 二次元でなく、各重さにおける価値の最大値を保持している
    let mut dp = vec![0; w + 1];
    for (we, v) in wv {
        // dp[i+we]の計算に前の値dp[i]を利用するため、rev()している
        for i in (0..=w - we).rev() {
            dp[i + we] = dp[i + we].max(dp[i] + v);
        }
    }
    println!("{}", dp[w]);
}
