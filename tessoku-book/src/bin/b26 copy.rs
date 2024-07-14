use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/45924260
fn main() {
    input! { n: usize, }
    let mut ps = vec![true;n+1];
    (2..=n).for_each(|i| if ps[i] {
        println!("{}", i);
        // n/iはどういう理屈か、
        (2..=n/i).for_each(|j| { ps[i*j] = false; } );
    });
}