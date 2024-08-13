use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize);n]
    }

    let mut cnt = 0;
    for i in 1..101 {
        for j in 1..101 {
            let mut cand = 0;
            for (a, b) in &ab {
                if *a <= i && i <= *a + k && *b <= j && j <= *b + k {
                    cand += 1;
                }
            }
            cnt = max(cnt, cand);
        }
    }
    println!("{}", cnt);
}
