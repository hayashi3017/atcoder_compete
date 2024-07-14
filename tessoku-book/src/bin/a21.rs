use proconio::input;

// TODO: 実装途中。時間切れのため回答をみる。index out of bounds error.
fn main() {
    input! {
        n: usize,
        pa: [(usize, usize);n]
    }

    let mut dp = vec![vec![0usize; n + 1]; n + 1];
    let mut p: Vec<usize> = vec![];
    let mut a: Vec<usize> = vec![];
    pa.iter().for_each(|v| {
        p.push(v.0);
        a.push(v.1);
    });

    // dbg!(&p);
    // dbg!(&a);

    for i in 0..n {
        for j in (0..n).rev() {
            if j < i {
                break;
            }

            let dpi = i + 1;
            let dpj = j + 1;
            let count = n - (dpj - dpi) + 1;
            if count == 0 {
                continue;
            }
            dbg!(count);

            let ls = if dpi <= p[count - 1] && p[count - 1] <= dpj {
                dp[dpi - 1][dpj] + a[count - 1]
            } else {
                dp[dpi - 1][dpj]
            };
            let rs = if dpi <= p[n - count] && p[n - count] <= dpj {
                dp[dpi][dpj + 1] + a[n - count]
            } else {
                dp[dpi][dpj + 1]
            };
            dbg!(ls);
            dbg!(rs);
            dp[dpi][dpj] = ls.max(rs);
        }
    }
    // dbg!(&dp);
}
