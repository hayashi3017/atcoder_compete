use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let sn = s.len();
    let tn = t.len();
    let mut dp = vec![vec![0; tn + 1]; sn + 1];

    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    for i in 0..sn {
        for j in 0..tn {
            let dpi = i + 1;
            let dpj = j + 1;
            if s[i] == t[j] {
                dp[dpi][dpj] = dp[dpi - 1][dpj]
                    .max(dp[dpi][dpj - 1])
                    .max(dp[dpi - 1][dpj - 1] + 1);
            } else {
                dp[dpi][dpj] = dp[dpi - 1][dpj].max(dp[dpi][dpj - 1]);
            }
        }
    }

    println!("{}", dp[sn][tn]);
}
