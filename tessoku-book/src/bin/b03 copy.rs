use itertools::Itertools;
use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/47748838
fn main() {
    input!(n: usize, a: [usize; n]);

    let cond = a
        .iter()
        // tips: get all combinations in array as tuple.
        .tuple_combinations()
        // tips: Tests if any element of the iterator matches a predicate.
        .any(|(a, b, c)| a + b + c == 1000);

    println!("{}", if cond { "Yes" } else { "No" });
}
