use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize;w];h],
        q: usize,
        abcd: [(usize, usize, usize, usize);q]
    }

    let mut sum = vec![vec![0; w]; h];

    // 横方向の累積和
    for i in 0..h {
        for j in 0..w {
            if j == 0 {
                sum[i][0] = x[i][0];
            } else {
                sum[i][j] = sum[i][j - 1] + x[i][j];
            }
        }
    }

    // 縦方向の累積和
    for i in 0..h {
        for j in 0..w {
            if i == 0 {
                continue;
            } else if j == 0 {
                sum[i][0] = sum[i - 1][0] + x[i][0];
            } else {
                sum[i][j] = sum[i][j] + sum[i - 1][j];
            }
        }
    }

    for (a, b, c, d) in abcd {
        let p4 = sum[c - 1][d - 1];
        let mut ans = 0;
        if a == 1 && b == 1 {
            ans = p4;
        } else if a == 1 {
            let p3 = sum[c - 1][b - 2];
            ans = p4 - p3;
        } else if b == 1 {
            let p2 = sum[a - 2][d - 1];
            ans = p4 - p2;
        } else {
            let p1 = sum[a - 2][b - 2];
            let p2 = sum[a - 2][d - 1];
            let p3 = sum[c - 1][b - 2];
            ans = p4 - (p2 + p3) + p1;
        }

        println!("{}", ans);
    }

    // [
    //     [2, 2, 2, 7, 8],
    //     [1, 1, 4, 4, 4],
    //     [0, 8, 13, 13, 15],
    //     [4, 5, 5, 5, 11],
    //     [0, 9, 11, 18, 18],
    // ]

    // [
    //     [2, 2, 2, 7, 8],
    //     [3, 3, 6, 11, 12],
    //     [3, 11, 19, 24, 27],
    //     [7, 16, 24, 29, 38],
    //     [7, 25, 35, 47, 56],
    // ]
}
