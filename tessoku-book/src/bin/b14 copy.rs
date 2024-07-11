use itertools::Itertools;
use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/47816696
fn main() {
    input!(n: usize, k: usize, a: [usize; n]);

    let (lhs, rhs) = a.split_at(n / 2);

    let rhs = rhs
        .iter()
        // https://cseweb.ucsd.edu/classes/sp22/cse223B-a/tribbler/itertools/trait.Itertools.html#method.powerset
        // https://zenn.dev/megeton/articles/fb6266bcb6aa1b#combination-%E3%81%AE%E9%87%8D%E3%81%AD%E6%8E%9B%E3%81%91-%E2%86%92-iter.powerset
        /*
            powerset()はv0.10.0から利用できる、v0.9.0では利用できないため注意
            https://docs.rs/itertools/0.9.0/itertools/index.html
         */
        .powerset()
        .map(|x| x.into_iter().sum::<usize>())
        // https://cseweb.ucsd.edu/classes/sp22/cse223B-a/tribbler/itertools/trait.Itertools.html#method.sorted
        .sorted()
        .collect::<Vec<_>>();

    let cond = lhs
        .iter()
        .powerset()
        .map(|x| x.into_iter().sum::<usize>())
        // tips: any()で二分探索してbool判定する
        .any(|x| rhs.binary_search(&(k - x)).is_ok());

    println!("{}", if cond { "Yes" } else { "No" });
}
