use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut p = vec![false; n + 1];

    for i in 0..=n {
        if i < a {
            continue;
        }

        if a <= i {
            if i < b {
                p[i] = !p[i - a];
            } else {
                p[i] = !p[i - a] || !p[i - b];
            }
        }
    }

    if p[n] {
        println!("First")
    } else {
        println!("Second")
    }
}
