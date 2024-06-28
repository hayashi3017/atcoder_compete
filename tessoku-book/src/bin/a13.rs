use proconio::input;

// TODO: 理解度が低いので見直しが必要そう
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize;n],
    }
    a.insert(0, 0);
    let mut x = vec![0; n];

    // おそらく考えやすいように1からn-1までの範囲を探索している
    for i in 1..=n - 1 {
        if i == 1 {
            x[0] = 1;
        } else {
            x[i] = x[i - 1];
        }

        // ここのx[i] + 1が重要な気がする。一つ先が条件をクリアするか確認している
        while x[i] < n && a[x[i] + 1] - a[i] <= k {
            x[i] += 1;
        }
    }

    let mut ans = 0;
    // 上で1からn-1の要素を利用する方針となっているので集計も合わせている
    for i in 1..=n - 1 {
        // 漸化式のようなもの？
        ans += x[i] - i;
    }

    println!("{}", ans);
}
