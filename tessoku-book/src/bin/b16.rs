use proconio::input;

fn main() {
    input! {
        n: isize,
        h: [isize;n]
    }

    let mut dp = vec![0isize, (h[0] - h[1]).abs()];

    // FIXME: この書き方は好きではない
    if n == 2 {
        println!("{}", dp[1]);
        return;
    }

    // n > 2
    for i in 2..n as usize {
        let min = (dp[i - 1] + (h[i - 1] - h[i]).abs()).min(dp[i - 2] + (h[i - 2] - h[i]).abs());
        dp.push(min);
    }
    println!("{}", dp.last().unwrap());
}
