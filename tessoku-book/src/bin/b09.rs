use proconio::input;

fn main() {
    input! {
        abcd: [(usize,usize,usize,usize)]
    }

    let mut v = vec![vec![0; 1501]; 1501];
    for (a, b, c, d) in abcd {
        v[b][a] += 1;
        v[b][c] -= 1;
        v[d][a] -= 1;
        v[d][c] += 1;
    }

    let mut v1 = vec![vec![0; 1501]; 1501];
    for i in 1..1501 {
        for j in 1..1501 {
            // vは0-1500のプロットなので調整
            v1[i][j] = v[i - 1][j - 1] + v1[i][j - 1];
        }
    }
    let mut ans = 0;
    for i in 1..1501 {
        for j in 1..1501 {
            v1[i][j] += v1[i - 1][j];
            if v1[i][j] > 0 {
                ans += 1;
            }
        }
    }
    // dbg!(&v);
    // [
    //     [0, 0, 0, 0, 0, 0],
    //     [0, 1, 0, -1, 0, 0],
    //     [0, 0, 1, 0, -1, 0],
    //     [0, -1, 0, 1, 0, 0],
    //     [0, 0, -1, 0, 1, 0],
    //     [0, 0, 0, 0, 0, 0],
    // ];
    // dbg!(&v1);
    // [
    //     [0, 0, 0, 0, 0, 0, 0],
    //     [0, 1, 1, 0, 0, 0, 0],
    //     [0, 1, 2, 1, 0, 0, 0],
    //     [0, 0, 1, 1, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0],
    // ];
    // 名前付き引数について
    // https://stackoverflow.com/questions/45356898/is-there-a-way-to-pass-named-arguments-to-format-macros-without-repeating-the-va
    // println!("{ans}");
    println!("{}", ans);
}
