use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        c: String,
    }

    if &c == "Red" {
        println!("{}", g.min(b));
    } else if &c == "Green" {
        println!("{}", b.min(r));
    } else {
        println!("{}", r.min(g));
    }
}
