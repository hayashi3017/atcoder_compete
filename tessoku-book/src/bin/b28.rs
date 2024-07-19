use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![0; n];
    ans[0] = 1;
    ans[1] = 1;
    const BASE_NUM: usize = 1000000007;
    for i in 2..n {
        ans[i] = (ans[i - 2] + ans[i - 1]) % BASE_NUM;
    }
    println!("{}", ans[n - 1]);
}
