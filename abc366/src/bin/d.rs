use proconio::input;

// 累積和を利用することは分かったが、最終形は見通せていなかったので累積和のデータ構造をミスしている
// 時間足らず実装途中で終了
fn main() {
    input! {
        n: usize,
        a: [[[usize;n];n];n],
        q: usize,
        lrxyz: [(usize, usize, usize, usize, usize, usize);q]
    }
    let mut ma = vec![0usize];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ma.push(ma.last().unwrap() + a[i][j][k]);
            }
        }
    }

    for (lx, rx, ly, ry, lz, rz) in lrxyz {
        let l = a[lx - 1][ly - 1][lz - 1];
        let r = a[rx - 1][ry - 1][rz - 1];
    }
}
