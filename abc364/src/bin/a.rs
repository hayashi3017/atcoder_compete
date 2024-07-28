use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    }

    let mut ans = true;

    for i in 0..n {
        if i == 0 || i == n - 1 {
            continue;
        }

        if s[i - 1] == "sweet" && s[i] == s[i - 1] {
            ans = false
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
