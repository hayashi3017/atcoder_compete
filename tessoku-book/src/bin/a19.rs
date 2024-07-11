use proconio::input;

fn main() {
    input! {
        N: usize,
        W: usize,
        wv: [(usize, usize);N]
    }

    // dbg!(&wv);
    let mut dp = vec![vec![-1isize; W + 1]; N + 1];
    dp[0][0] = 0;
    let mut w = vec![0usize; N];
    let mut v = vec![0usize; N];
    for (i, wv) in wv.iter().enumerate() {
        w[i] = wv.0;
        v[i] = wv.1;
    }

    for i in 0..N {
        for j in 0..=W {
            let dpi = i + 1;
            // 品物iまでの最大値を常に保持していくので、追加分の品物のみ比較して更新していけばよい
            if w[i] <= j {
                dp[dpi][j] = dp[dpi - 1][j].max(dp[dpi - 1][j - w[i]] + v[i] as isize);
            } else {
                dp[dpi][j] = dp[dpi - 1][j];
            }
        }
    }

    // tips: 二次元配列の値のうち最大値を得るために一度フラットにする
    let max = dp.iter().flat_map(|v| v.iter()).max().unwrap();
    // dbg!(&dp);
    println!("{}", max)
}
