use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        mut a: [usize;n]
    }

    a.insert(k, x);
    let ans = a
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans)
}
