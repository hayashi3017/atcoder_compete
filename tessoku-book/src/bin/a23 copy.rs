use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/45534644
fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [[usize; n]; m],
    }

    let xs = xs
        .into_iter()
        // ビットシフトとビットORを用いて、各配列を1つの整数に変換する
        .map(|xs| xs.into_iter().enumerate().fold(0, |y, (i, x)| y | x << i))
        .collect::<Vec<_>>();

    // tips: 1 << n は2のn乗
    let mut dp = vec![None; 1 << n];
    dp[0] = Some(0);

    for i in 0..dp.len() {
        for &x in &xs {
            // 現在の状態iと次の状態i | xをビットORで計算する
            dp[i | x] = dp[i | x].into_iter().chain(dp[i].map(|x| x + 1)).min();
        }
    }

    println!("{}", dp.last().unwrap().unwrap_or(-1));
}

// ビット演算
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8fc1190f049964573fe2977e837c4d4a