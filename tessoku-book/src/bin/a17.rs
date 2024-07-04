use std::{collections::VecDeque, iter::FromIterator};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n -1],
        b: [usize; n -2],
    }

    // tips: 配列の長さを揃えるために配列先頭へ要素を追加する、Vec::append()やVec::insert()よりもVecDequeを利用する方が効率的らしい
    let mut deque_a = VecDeque::from_iter(a);
    deque_a.push_front(0);
    let mut deque_b = VecDeque::from_iter(b);
    deque_b.push_front(0);
    deque_b.push_front(0);
    // dbg!(&deque_a);
    // dbg!(&deque_b);
    let mut dp = vec![0, deque_a[1]];

    for i in 2..n {
        let min = (dp[i - 1] + deque_a[i]).min(dp[i - 2] + deque_b[i]);
        dp.push(min);
    }

    // dbg!(&dp);
    let mut ans: Vec<usize> = vec![];
    // tips: 反復処理の基数が不規則に変化する場合、for文ではなくloopと基数を利用する
    let mut p = n;
    loop {
        ans.push(p);
        if p == 1 {
            break;
        }
        let i = p - 1;
        if dp[i] == dp[i - 1] + deque_a[i] {
            p -= 1;
        } else {
            p -= 2;
        }
    }
    // for i in (2..n).rev() {
    //     if dp[i] == dp[i-1] + deque_a[i] {
    //         ans.push(i);
    //     } else {
    //         ans.push(i);
    //     }
    // }

    println!("{}", ans.len());
    ans.reverse();
    // dbg!(&ans);
    ans.iter().for_each(|v| {
        print!("{} ", v);
    });
}

// https://qiita.com/muumu/items/c2611ba0fdeabe496727#splitwhitespace
// cargo compete test した際の下記表示について
// note:
// whitespace-separated words matched. try setting `match` to `SplitWhitespace`