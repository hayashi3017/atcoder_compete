// https://tanakh.hatenablog.com/entry/2020/02/01/160442
// https://docs.rs/memoise/latest/memoise/
use memoise::memoise;
use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/54947083
fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }
    if first_player_win(n, a, b) {
        println!("First");
    } else {
        println!("Second");
    }
}

#[memoise(n)]
fn first_player_win(n: u32, a: u32, b: u32) -> bool {
    if n < a && n < b {
        return false;
    }
    let mut res = false;
    if n >= a {
        res |= !first_player_win(n - a, a, b);
    }
    if n >= b {
        res |= !first_player_win(n - b, a, b);
    }
    res
}
