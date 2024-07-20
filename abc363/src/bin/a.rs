use proconio::input;

fn main() {
    input! {
        r: usize,
    }
    if r < 100 {
        println!("{}", 100 - r)
    } else if r < 200 {
        println!("{}", 200 - r)
    } else if r < 300 {
        println!("{}", 300 - r)
    }
}
