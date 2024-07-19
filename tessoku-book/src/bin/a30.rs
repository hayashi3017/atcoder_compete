use num_traits::Pow;
use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
    }
    const MOD: usize = 1e9 as usize + 7;
    let mut na = 1usize;
    for i in 1..=n {
        na *= i;
        na %= MOD;
    }
    let mut ra = 1usize;
    for i in 1..=r {
        ra *= i;
        ra %= MOD;
    }
    for i in 1..=(n - r) {
        ra *= i;
        ra %= MOD;
    }

    // dbg!(na);
    // dbg!(ra);
    // 計算としては合っているはずだが、オーバーフローする
    // println!("{}", na * ra.pow((MOD - 2) as u32) % MOD);

    // オーバーフローを回避するために繰り返し二乗法を利用する
    let mut ans = 1;
    let mut count = MOD - 2;
    while count > 0 {
        if count & 1 == 1 {
            ans = ans * ra % MOD;
        }
        count >>= 1;
        ra = ra * ra % MOD;
    }
    // dbg!(ans);
    println!("{}", na * ans % MOD);
}
