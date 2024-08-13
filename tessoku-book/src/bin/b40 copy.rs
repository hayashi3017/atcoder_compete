use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/55586555
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut cnt = vec![0; 100];
    for a in a {
        // 求めたい解は100で割り切れる数なので、100で割った余りをもっておけばよい
        cnt[a % 100] += 1;
    }
    let mut ans = 0_i64;
    // 余りが0となる数のうち、2つ選んだときのケース数
    ans += cnt[0] * (cnt[0] - 1) / 2;
    // 余りが50となる数のうち、2つ選んだときのケース数
    ans += cnt[50] * (cnt[50] - 1) / 2;
    for i in 1..50 {
        ans += cnt[i] * cnt[100 - i];
    }
    println!("{}", ans);
}
