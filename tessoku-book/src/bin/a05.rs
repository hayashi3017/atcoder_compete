use proconio::input;

fn main() {
    input! {
        n: isize,
        k: isize,
    }

    // let mut a = 0;
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a8102e1986ab5f1aa41b1f6c91bfc685
    // (1..=n).permutations(3).for_each(|d| {
    //     if d.iter().sum::<usize>() == k {
    //         a += 1;
    //     }
    // });

    let mut a = 0;
    for i in 1..=n {
        for j in 1..=n {
            let m = k - (i + j);
            if 0 < m && m <= n {
                a += 1;
            }
        }
    }

    println!("{}", a);
}
