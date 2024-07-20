use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        mut l: [usize;n]
    }

    l.sort();

    let mut count = 0;
    while l[n - p] < t {
        count += 1;
        l = l.iter().map(|v| v + 1).collect();
    }

    println!("{}", count);
}
