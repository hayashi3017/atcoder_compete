use proconio::*;

// https://atcoder.jp/contests/tessoku-book/submissions/43319433
fn main() {
    input! {n:i32};
    // https://doc.rust-lang.org/std/fmt/index.html#syntax
    // https://doc.rust-lang.org/std/fmt/index.html#formatting-traits
    println!("{:010b}", n);
}
