use num_traits::Pow;
use proconio::input;

fn main() {
    input! {
        n: f64,
    }

    let mut left = 0.0;
    let mut right = n;
    // whileの条件に注意
    while right - left > 0.001 {
        let mid = (left + right) / 2.0;
        let c = mid.pow(3) + mid;
        if c <= n {
            left = mid;
        } else {
            right = mid;
        }
    }
    // dbg!(left);
    // dbg!(right);

    // 任意の小数点数で出力する
    println!("{:.6}", right);
}
