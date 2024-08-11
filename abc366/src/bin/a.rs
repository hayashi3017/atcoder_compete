use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: usize,
    }
    let r = n - t - a;
    if t > a + r || a > t + r {
        println!("Yes");
    } else {
        println!("No");
    }
}
