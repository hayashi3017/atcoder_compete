use proconio::input;

// TODO: 途中・・消化しきれていない
fn main() {
    input! {
        N: usize,
        W: usize,
        wv: [(isize, isize);N]
    }

    dbg!(&wv);
    let mut dp = vec![vec![-1isize; W]; N];
    dp[0][0] = 0;

    for i in 1..N {
        for j in 0..W {
            let wv_s = &wv[0..i - 1];
            // FIXME: 無駄がある、これだとナップザックに入る可能性のあるアイテムを毎回すべて確認している。dpはその時点での最大値を保持するので累積和のようにその時点のjを基準に1つだけ確認すればよい
            let filtered = wv_s
                .iter()
                .filter(|(ws, _)| *ws as usize <= j)
                .collect::<Vec<_>>();
            // dbg!(&filtered);
            // 重さj以下で何らかをナップサックへ入れることができるか
            if filtered.len() != 0 {
                if dp[i - 1][j] != -1
                    || wv_s
                        .iter()
                        .any(|(ws, _vs)| dp[i - 1][j - *ws as usize] != -1)
                {
                    let max_v = wv_s
                        .iter()
                        .filter(|(ws, _)| *ws as usize == j)
                        .map(|(_, vs)| vs)
                        .max()
                        .unwrap();
                    dp[i][j] = dp[i - 1][j] + *max_v;
                }
            } else {
                if dp[i - 1][j] != -1 {
                    // 0以外ないかもしれない
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
    }

    dbg!(&dp);
}
