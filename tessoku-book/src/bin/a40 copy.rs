use proconio::{input, marker::Usize1};

// https://atcoder.jp/contests/tessoku-book/submissions/55586503
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut cnt = vec![0; 100];
    for a in a {
        cnt[a] += 1;
    }
    let mut ans = 0_i64;
    for cnt in cnt {
        ans += cnt * (cnt - 1) * (cnt - 2) / 6;
    }
    println!("{}", ans);
}
