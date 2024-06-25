use proconio::input;

// https://atcoder.jp/contests/tessoku-book/tasks/math_and_algorithm_al
fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize);n]
        // lr: [(usize, usize)]
    }

    // tvはtではなくt+1まで取りうる配列であることに注意、t+1番目の値は出力には利用しないが入力では受け入れる値
    let mut tv: Vec<isize> = vec![0; t + 1];
    // 冗長な記述のため変更
    // for i in 0..n {
    //     let (l, r) = lr[i];

    //     tv[l] += 1;
    //     tv[r] -= 1;
    // }
    for (l, r) in lr {
        tv[l] += 1;
        tv[r] -= 1;
    }
    // dbg!(&tv);

    // 累計を配列に持つ必要はない、この問題では答え1ケース出力するのみで十分
    // let mut sumv: Vec<usize> = vec![0; t];
    // for i in 0..t {
    //     if i == 0 {
    //         sumv[0] = tv[0] as usize;
    //     } else {
    //         sumv[i] = (sumv[i - 1] as isize + tv[i]) as usize;
    //     }
    //     println!("{}", sumv[i]);
    // }
    let mut ans = 0;
    for i in 0..t {
        ans += tv[i];
        println!("{}", ans);
    }
}
