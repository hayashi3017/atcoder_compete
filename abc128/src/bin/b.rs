use itertools::Itertools;
use proconio::input;

// https://atcoder.jp/contests/abc128/submissions/49048662
fn main() {
    input! {
        n: usize,
        sn: [(String, i8); n]
    }

    // // indexed
    // for d in &sn {}
    // // sort
    // for (s, n) in &sn {}

    println!(
        "{}",
        sn.iter()
            .enumerate()    // indexed by enumerate method
            .sorted_by_key(|(_, (s, p))| (s, -*p))   // tips: タプル(s, -*p)をソートすると、s(String)の降順、p(数値)の降順でソートできる様子
            .map(|(i, _)| i + 1)    // convert indexed number to natural number
            .join("\n")
    );
}
