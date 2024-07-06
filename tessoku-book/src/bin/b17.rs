use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize;n]
    }

    let mut dp: Vec<isize> = vec![0, (h[1] - h[0]).abs()];

    for i in 2..n {
        let min = (dp[i - 1] + (h[i - 1] - h[i]).abs()).min(dp[i - 2] + (h[i - 2] - h[i]).abs());
        dp.push(min);
    }

    // dbg!(&dp);

    let mut ans = vec![];
    let mut pos = n;
    loop {
        ans.push(pos);

        if pos == 1 {
            break;
        }
        let i = pos - 1;

        if dp[i] == dp[i - 1] + (h[i - 1] - h[i]).abs() {
            pos -= 1;
        } else {
            pos -= 2;
        }
    }

    println!("{}", ans.len());
    ans.reverse();
    // join()はVec<&str>に対して実装されている、Join<&str>
    // https://doc.rust-lang.org/stable/std/primitive.slice.html#method.join
    println!("{}", ans.iter().map(|num| num.to_string()).join(" "));
}
