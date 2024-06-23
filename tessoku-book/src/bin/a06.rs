use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize;n],
        lr: [(usize, usize);q]
    }

    // 全ての要素で最初から累積和を求めているので非効率
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a8102e1986ab5f1aa41b1f6c91bfc685
    // let v = (0..n)
    //     .map(|i| {
    //         let mut sum = 0;
    //         (0..=i).for_each(|j| sum += a[j]);
    //         sum
    //     })
    //     .collect_vec();

    // tips: 前日の累積和を利用して累積和を求める
    let mut v: Vec<usize> = Vec::new();
    for i in 0..n {
        if i == 0 {
            v.push(a[0]);
            continue;
        }
        v.push(v[i - 1] + a[i]);
    }

    (0..q).for_each(|i| {
        let mut ans = 0;
        let (start, end) = lr[i];
        if start == 1 {
            ans += v[end - 1];
        } else {
            ans += v[end - 1] - v[(start - 1) - 1];
        }
        println!("{}", ans);
    })
}
