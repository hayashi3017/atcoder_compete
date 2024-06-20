use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/42863469
fn main() {
    input! {
        n: usize
    }

    (0..10)
        .rev()
        .for_each(|i| print!("{}", n / 2usize.pow(i) % 2));

    // for i in 0..10 {
    //     let j = 9 - i;
    //     print!("{}", n / 2usize.pow(j) % 2);
    // }

    println!("");
}
