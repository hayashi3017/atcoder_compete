use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let v = s.chars().collect::<Vec<char>>();
    for i in 2..n {
        if v[i] == v[i - 1] && v[i] == v[i - 2] {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
