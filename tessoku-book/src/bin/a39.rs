use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize);n]
    }

    // tips: 要素のある値を基準にソートする
    // lr.sort_by_key(|a| a.1);
    // tips: 要素の複数値を基準に順序をつけてソートする
    lr.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    let mut ans = 0;
    let mut t = 0;
    for i in 0..n {
        let (l, r) = lr[i];
        if t <= l {
            ans += 1;
            t = r;
        }
    }

    println!("{}", ans);
}
