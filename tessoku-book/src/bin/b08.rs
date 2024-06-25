use proconio::input;

// https://atcoder.jp/contests/tessoku-book/submissions/51124385
fn main() {
    input! {
        xy: [(usize, usize)],
        abcd: [(usize, usize, usize, usize)],
    }
    // tips: 累積和では計算に前項を利用するため、事前に配列を1だけ長くしておけば初段計算の場合分けを避けられる
    let mut t = vec![vec![0; 1501]; 1501];
    for (x, y) in xy {
        t[y][x] += 1;
    }
    // too big array to debug. this line causes Timelimit Exceeded (5000 ms)
    // dbg!(&t);
    let mut m = vec![vec![0usize; 1501]; 1501];
    for y in 1..1501 {
        for x in 1..1501 {
            // 累積和の計算
            // 縦横2回に分ける必要がなく一度にできる場合は1回で行う、要領は答えを出力する時と同じ
            m[y][x] = t[y][x] + m[y - 1][x] + m[y][x - 1] - m[y - 1][x - 1];
        }
    }
    for (a, b, c, d) in abcd {
        println!("{}", m[d][c] + m[b - 1][a - 1] - m[d][a - 1] - m[b - 1][c]);
    }
}
