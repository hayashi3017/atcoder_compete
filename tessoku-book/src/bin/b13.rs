use proconio::input;

// TODO: 未解決
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n]
    }

    // tips: 累積和を求める
    let mut a1: Vec<usize> = a
        .iter()
        .scan(0, |state, &v| {
            *state += v;
            Some(*state)
        })
        .collect();
    // dbg!(&a1);

    let mut r = vec![0usize; n];
    for i in 0..(n - 1) {
        if i == 0 {
            r[0] = 0;
        } else {
            r[i] = r[i - 1];
        }

        while r[i] < n - 1 && a1[r[i] + 1] - a1[i] + a[i] <= k {
            r[i] += 1;
        }
    }

    // dbg!(&r);
    let mut ans = 0;
    for i in 0..(n - 1) {
        ans += r[i] - i;
        // dbg!(ans);
    }

    // TODO: 冗長なので他に組み込みたい・・・
    for i in 0..n {
        if a[i] <= k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
