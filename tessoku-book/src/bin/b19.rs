#![allow(non_snake_case)]
use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/36272934
#[proconio::fastout]
fn main() {
    input! {
        N: usize,
        W: usize,
        wvs: [(usize, usize); N],
    }
    // dpは1次元で問題ない、品物の選び方に制限がないため
    // 価値に対する重さの最小値を保持する
    let mut dp = vec![W + 1; 100_001];
    dp[0] = 0;
    for (w, v) in wvs {
        for vn in (v..=100_000).rev() {
            // vnが100_000から入ったとして、dp[i]の初期値はW + 1なら、dp[vn - v]もW + 1なのでdp[vn - v] + wが最小値となるケースは起こりえないのでは？？
            // TODO: なぜ動作するのか確認する
            dp[vn] = dp[vn].min(dp[vn - v] + w);
        }
    }
    let mut ans = 100_000;
    while dp[ans] > W {
        ans -= 1;
    }
    println!("{}", ans);
}
