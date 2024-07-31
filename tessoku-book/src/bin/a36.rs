use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    if (n - 1) * 2 <= k && k & 1 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
