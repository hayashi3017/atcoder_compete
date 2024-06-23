use proconio::input;

// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ce
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        q: usize,
        lr: [(usize, usize);q]
    }

    // 初項と他項を区別してpushすることで配列を作成する方法
    // let mut v: Vec<usize> = Vec::new();
    // (0..n).for_each(|i| {
    //     if i == 0 {
    //         v.push(a[0]);
    //     } else {
    //         v.push(v[i - 1] + a[i]);
    //     }
    // });

    // 一度同じロジックで配列を作成して、後から不要な要素を削除することで配列を作成する方法
    let mut v = vec![0];
    (0..n).for_each(|i| {v.push(v.last().unwrap() + a[i])});
    v.remove(0);

    // TODO: RunTimeErrorの原因を調査する
    for i in 0..q {
        let (start, end) = lr[i];
        let si = start - 1;
        let ei = end - 1;

        let range = end - start + 1;
        let hit = v[ei] - v[si - 1];
        let miss = range - hit;

        if miss < hit {
            println!("win");
        } else if hit == miss {
            println!("draw");
        } else {
            println!("lose");
        }
    }
}
