use proconio::{input, marker::Usize1};

// https://atcoder.jp/contests/abc366/submissions/56543446
fn main() {
    input! {
        n: usize,
        a: [[[u64; n]; n]; n],
        q: usize,
        lxrxlyrylzrz: [(Usize1, usize, Usize1, usize, Usize1, usize); q],
    }
    let mut acc = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                acc[i + 1][j + 1][k + 1] = acc[i][j + 1][k + 1] + acc[i + 1][j][k + 1] + acc[i + 1][j + 1][k] - acc[i + 1][j][k] - acc[i][j + 1][k] - acc[i][j][k + 1] + acc[i][j][k] + a[i][j][k];
            }
        }
    }
    for (lx, rx, ly, ry, lz, rz) in lxrxlyrylzrz {
        println!("{}", acc[rx][ry][rz] - acc[lx][ry][rz] - acc[rx][ly][rz] - acc[rx][ry][lz] + acc[lx][ly][rz] + acc[lx][ry][lz] + acc[rx][ly][lz] - acc[lx][ly][lz]);
    }
}
