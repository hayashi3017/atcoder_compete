use itertools::Itertools;

// https://atcoder.jp/contests/abc363/submissions/55780628
fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        s: String,
    }
    let res = s
        .chars()
        .permutations(n)
        .unique()
        .filter(|x| x.windows(k).all(|w| !w.iter().eq(w.iter().rev())))
        .count();

    println!("{res}");
}
