use proconio::input;

fn main() {
    // input! {
    //     n: isize,
    //     m: isize,
    //     ks: [(isize, [isize]); m],
    //     p: [isize; m],
    // }

    // bit探索で全探索してonになることを確認すればいい

    // https://atcoder.jp/contests/abc128/submissions/22453151
    // あまり美しくはない、bit探索していない？
    input! {n:i8,m:usize,ksm:[[usize];m],pm:[i8;m]}
    println!(
        "{}",
        (0..1 << n).fold(0, |ans, i| {
            if (0..m).all(|x| {
                (ksm[x]
                    .iter()
                    .map(|v| (i & 1 << (v - 1) != 0) as i8)
                    .sum::<i8>())
                    % 2
                    == pm[x]
            }) {
                ans + 1
            } else {
                ans
            }
        })
    )
}
