use proconio::input;

fn main() {
    input! {
        _: usize,
        k: usize,
        s: String,
    }

    if s.chars().fold(0, |acc, v| {
        if v == '1' {
            return acc + 1;
        }
        return acc;
    }) % 2
        == k % 2
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
