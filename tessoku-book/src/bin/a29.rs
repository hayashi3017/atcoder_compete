use proconio::input;

// 繰り返し二乗法
fn main() {
    input! {
        mut a: usize,
        mut b: usize
    }

    // tips: 大きい数値は指数表記で表現する、指数exponentの略
    const MOD: usize = 1e9 as usize + 7;
    // 累乗の結果を格納するための変数、b=0とするとans=1なので1始まりでOK
    let mut ans = 1;
    while b > 0 {
        // dbg!(format!("{:b}", b));
        // dbg!(ans);
        // dbg!(a);

        // tips: ビット演算で奇数判定
        if b & 1 == 1 {
            ans = ans * a % MOD;
        }
        // tips: ビット演算の右シフトで計算、2で割ることと同義
        b >>= 1;
        a = a * a % MOD;
    }
    println!("{}", ans);
}
