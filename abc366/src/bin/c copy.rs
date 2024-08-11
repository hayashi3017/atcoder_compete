use std::collections::BTreeMap;

use proconio::input;

// https://atcoder.jp/contests/abc366/submissions/56533458
fn main() {
    input! {
        q: usize,
    }
    let mut map = BTreeMap::new();
    for _ in 0..q {
        input!(op: u8);
        if op == 1 {
            input!(x: u32);
            *map.entry(x).or_insert(0) += 1;
        } else if op == 2 {
            input!(x: u32);
            *map.get_mut(&x).unwrap() -= 1;
            if map[&x] == 0 {
                map.remove(&x);
            }
        } else {
            println!("{}", map.len());
        }
    }
}
