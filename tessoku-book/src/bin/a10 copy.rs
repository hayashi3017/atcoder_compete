use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/42061757
fn main() {
    input! {
        n: i32,
        a: [i32;n],
        d: i32,
        lr: [(usize, usize);d],
    }

    for (i, j) in lr {
        println!(
            "{}",
            std::cmp::max(&a[..i - 1].iter().max(), &a[j..].iter().max()).unwrap()
        );
    }
}
