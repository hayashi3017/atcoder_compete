use proconio::{fastout, input};

// https://atcoder.jp/contests/tessoku-book/submissions/55586820
#[fastout]
fn main() {
    input! {
        mut x: u32,
        mut y: u32,
    }
    let mut ans = vec![];
    while x > 1 || y > 1 {
        ans.push((x, y));
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }
    ans.reverse();
    println!("{}", ans.len());
    for (x, y) in ans {
        println!("{} {}", x, y);
    }
}
