use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/55610447
fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut ans = 0;
    for i in [-1, 1] {
        for j in [-1, 1] {
            let mut tmp = 0;
            for &(a, b) in &ab {
                if a * i + b * j > 0 {
                    tmp += a * i + b * j;
                }
            }
            ans = ans.max(tmp);
        }
    }
    println!("{}", ans);
}
