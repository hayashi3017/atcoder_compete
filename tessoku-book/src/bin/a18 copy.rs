use bitset_fixed::BitSet;
// TODO: これの解説をしてもいいかもしれない
// https://docs.rs/crate/bitset-fixed/0.1.0/source/

// https://atcoder.jp/contests/tessoku-book/submissions/36143556
fn main() {
    proconio::input! {n: usize, s: usize, a: [usize; n]};
    let mut b = BitSet::new(s + 1);
    // dbg!(&b);
    // [src/bin/a18.rs:6] &b = BitSet {
    //     buf: [
    //         0,
    //     ],
    //     size: 12,
    // }
    b.set(0, true);
    // dbg!(&b);
    // [src/bin/a18.rs:8] &b = BitSet {
    //     buf: [
    //         1,
    //     ],
    //     size: 12,
    // }
    a.iter().for_each(|&v| b.shl_or(v));
    // dbg!(&b);
    // [src/bin/a18.rs:10] &b = BitSet {
    //     buf: [
    //         2043,
    //     ],
    //     size: 12,
    // }
    let r = match b[s] {
        true => "Yes",
        false => "No",
    };
    println!("{}", r);
}
