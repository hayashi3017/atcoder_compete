use proconio::input;
use std::cmp::Ordering;

// https://atcoder.jp/contests/tessoku-book/submissions/54909680
fn main() {
    input! {
        n: usize,
        x: u32,
        a: [u32; n]
    };

    let ans = binary_search(&a, x, 0).unwrap();

    println!("{}", ans + 1);
}

fn binary_search<T: std::cmp::Ord + std::fmt::Debug>(
    xs: &[T],
    x: T,
    i: usize,
) -> Result<usize, usize> {
    if xs.len() == 1 {
        return match x.cmp(&xs[0]) {
            Ordering::Equal => Ok(i),
            Ordering::Less => Err(i),
            Ordering::Greater => Err(i + 1),
        };
    }
    let mid = (xs.len()) / 2;
    match x.cmp(&xs[mid]) {
        Ordering::Equal => Ok(i + mid),
        Ordering::Less => binary_search(&xs[..mid], x, i),
        Ordering::Greater => binary_search(&xs[mid..], x, i + mid),
    }
}
