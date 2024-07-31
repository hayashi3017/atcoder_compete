use proconio::*;
use superslice::*;

// https://atcoder.jp/contests/abc364/submissions/56011567
fn main() {
    input! { n: usize, q: usize, mut a: [i64; n] }
    a.sort();
    for _ in 0..q {
        input! { b: i64, k: usize }
        // 二分探索の探索範囲、0から2^60までで十分という考え方っぽい？　10^8までカバーできればよいので2^30（約10^9）で十分かも？
        let (mut l, mut r) = (0, 1i64 << 60);
        let ll = a.lower_bound(&b);
        let rr = a.upper_bound(&b);
        // どのようなケースを想定しているか不明、kは数直線上の数字とは比較できないと思うが違うのか
        if rr - ll >= k {
            println!("0");
            continue;
        }
        // 二分探索
        while r - l > 1 {
            let mid = (l + r) / 2;
            let ll = a.lower_bound(&(b - mid));
            let rr = a.upper_bound(&(b + mid));
            // cntはaの全長からはじまり、徐々に狭まっていく値のはず
            let cnt = rr - ll;
            if cnt < k {
                l = mid;
            } else {
                r = mid;
            }
        }
        let ll = a.lower_bound(&(b - l));
        let rr = a.lower_bound(&(b + l));
        if rr - ll == k {
            println!("{l}");
        } else {
            println!("{}", l + 1);
        }
    }
}


/*
// https://atcoder.jp/contests/abc364/editorial/10549
    本問題のように「k 番目に○○な△△を求める」という問題では、しばしば二分探索が解法の初手として用いられます。実際、本問題においても二分探索を用いることができます。具体的には、～

    解説をみるとよく理解できる。
    https://www.youtube.com/watch?v=hmWfjnLMPZg

    単調性がある、つまりどこかに境界があることに注目すること
*/
