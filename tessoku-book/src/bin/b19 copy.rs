use proconio::input;

// FIXME: 自作、要修正
fn main() {
    input! {
        N: usize,
        W: usize,
        wv: [(usize, usize);N]
    }

    // dp[i][j]は、品物iまでのうち価値がjとなるような重さの最小値
    /* p[i][j]となるには
       品物i-1の段階で価値がjとなっており、品物iを追加しない。
       品物i-1の段階で価値がjとなっており、品物iを追加する。最小値は dp[i][j] = min(dp[i-1][j], w[i])
       品物i-1の段階で価値がj-v[i]となっており、品物iを追加する。最小値は dp[i][j] = min(dp[i-1][j-v[i]] + w[i], dp[i-1][j])
    */
    let mut dp = vec![vec![-1isize; 100001]; N + 1];
    let mut w: Vec<usize> = vec![];
    let mut v: Vec<usize> = vec![];
    wv.iter().for_each(|wv| {
        w.push(wv.0);
        v.push(wv.1);
    });

    // dbg!(w);
    // dbg!(v);

    dp[0][0] = 0;
    for i in 0..N {
        for j in 0..100000usize {
            let dpi = i + 1;
            let dpj = j + 1;
            if v[i] <= j {
                dp[dpi][dpj] = dp[dpi - 1][dpj]
                    .min(w[i] as isize)
                    .min(dp[dpi - 1][dpj - v[i]] + w[i] as isize);
            } else {
                dp[dpi][dpj] = dp[dpi - 1][dpj].min(w[i] as isize);
            }
        }
    }

    dbg!(dp);

    // 実行時間の関係で、出力ができないと思われる。。
}
