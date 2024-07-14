use proconio::input;

// ユークリッドの互除法
// [ユークリッドの互除法 - Wikipedia](https://ja.wikipedia.org/wiki/%E3%83%A6%E3%83%BC%E3%82%AF%E3%83%AA%E3%83%83%E3%83%89%E3%81%AE%E4%BA%92%E9%99%A4%E6%B3%95)
fn main() {
    input! {
        mut a: usize,
        mut b: usize
    }

    while a != 0 && b != 0 {
        if a <= b {
            b = b % a;
        } else {
            a = a % b;
        }
    }

    if a == 0 {
        println!("{}", b);
    } else {
        println!("{}", a);
    }
}
