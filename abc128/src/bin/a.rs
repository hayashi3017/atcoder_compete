use proconio::input;

fn main() {
    input! {
        a: usize,
        p: usize,
    }

    let sum = a * 3 + p;
    println!("{}", sum / 2);
}
