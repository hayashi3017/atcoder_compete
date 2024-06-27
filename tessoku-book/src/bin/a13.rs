use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n],
    }

    // TODO: 未解決。原因不明。
    let mut ans = 0;
    let mut x = 0;
    for i in 0..n {
        while a[x + 1] - a[i] <= k  {
            if x < n - 2 {
                x += 1;
            }
            ans += 1;
        }
    }

    println!("{}", ans);
}
