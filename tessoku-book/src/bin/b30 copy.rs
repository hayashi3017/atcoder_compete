#![allow(non_snake_case, unused)]

// https://atcoder.jp/contests/tessoku-book/submissions/55499749

use itertools::Itertools;
use proconio::{marker::*, source::line::LineSource, *};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use std::io::*;
use std::{cmp::*, vec};
use std::{collections::*, fmt::format};
use superslice::Ext;

const MOD: i64 = 1000000007;

fn factorial(a: i64, p: i64) -> i64 {
    if a == 0 {
        return 1;
    }
    let mut n = 1;
    let mut a = a;
    while a > 1 {
        n *= a;
        n %= p;
        a -= 1;
    }
    n
}

/// x^p mod MODを繰り返し二乗法により求める
/// O(log(p))
fn modpow(mut x: i64, mut p: i64, mod_num: i64) -> i64 {
    let mut ret = 1;
    while p > 0 {
        if p & 1 == 1 {
            ret *= x;
            ret %= mod_num;
        }
        x *= x;
        x %= mod_num;
        p >>= 1;
    }
    ret
}

fn main() {
    input! {
        H: i64,
        W: i64,
    }
    // 下H-1回をいつやるか選ぶ
    let N = H + W - 2;
    let a = factorial(N, MOD);
    let b = factorial(H - 1, MOD) * factorial(W - 1, MOD) % MOD;
    let inv_b = modpow(b, MOD - 2, MOD);

    println!("{}", a * inv_b % MOD);
}
