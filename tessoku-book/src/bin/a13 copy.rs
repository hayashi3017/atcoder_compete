use superslice::Ext;

// https://atcoder.jp/contests/tessoku-book/submissions/43070441
fn main() {
    proconio::input! { n:usize,k:usize,an:[usize;n] }
    let ans = (0..n).map(|i| an[i + 1..].upper_bound(&(an[i] + k)));
    println!("{}", ans.sum::<usize>());
}
