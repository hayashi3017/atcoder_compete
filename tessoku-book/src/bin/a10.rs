use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
        lr: [(usize,usize)]
    }

    // TODO: reduce()は使用できない、別の書き方で修正する
    // TODO: reduce()でのロジックはテストパスしないので破綻している様子
    let mut vec = vec![];
    let _ = &a.iter().reduce(|acc, v| {
        let n = acc.max(v);
        vec.push(*n);
        n
    });
    let mut vec2 = vec![];
    let _ = &a.iter().rev().reduce(|acc, v| {
        let n = acc.max(v);
        vec2.push(*n);
        n
    });

    for (l, r) in lr {
        let m1 = vec[0..l-1].iter().max().unwrap();
        let m2 = vec2[r..n].iter().max().unwrap();
        println!("{}", m1.max(m2));
    }
}

// https://atcoder.jp/contests/tessoku-book/submissions/36927497
