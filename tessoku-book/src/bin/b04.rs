use proconio::input;

fn main() {
    input!(n: String);

    // tips: 文字列から10進数数値を取得する、文字の基数を指定できる
    // https://doc.rust-lang.org/std/primitive.usize.html#method.from_str_radix
    println!("{}", usize::from_str_radix(&n, 2).unwrap());
}