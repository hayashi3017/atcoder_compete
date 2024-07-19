// -*- coding:utf-8-unix -*-

use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/55242887
fn main() {
    input! {
        n: i128,
        r: i128,
    }
    let mod_n = 1000000007;
    println!(
        "{}",
        mod_div(
            mod_factorial(n, mod_n),
            mod_factorial(r, mod_n) * mod_factorial(n - r, mod_n),
            mod_n
        )
    );
}

fn mod_div(a: i128, b: i128, mod_n: i128) -> i128 {
    return (a * mod_pow(b, mod_n - 2, mod_n)) % mod_n;
}
fn mod_factorial(mut a: i128, mod_n: i128) -> i128 {
    let mut res = 1;
    while a > 1 {
        res = (res * a) % mod_n;
        a -= 1;
    }
    return res;
}

fn mod_pow(a: i128, b: i128, mod_n: i128) -> i128 {
    let mut aas = vec![a];
    for _ in 0..30 {
        aas.push((aas.last().unwrap() * aas.last().unwrap()) % mod_n);
    }
    let binary_b: Vec<char> = format!("{:b}", b).chars().rev().collect();
    let mut res = 1;
    for (i, bb) in binary_b.iter().enumerate() {
        if *bb == '1' {
            res = (res * aas[i]) % mod_n;
        }
    }
    return res;
}
