// https://atcoder.jp/contests/tessoku-book/submissions/41494606

fn main() {
    proconio::input! {
        n:usize,a:[usize;n-1],b:[usize;n-2]
    }
    // dpで今回必要なのは2つ（前の前、前）なのでタプルで十分、配列で持つ必要はない
    let mut c = (0, a[0]);
    for i in 2..n {
        c = (c.1, (c.0 + b[i - 2]).min(c.1 + a[i - 1]));
    }
    println!("{}", c.1);
}
