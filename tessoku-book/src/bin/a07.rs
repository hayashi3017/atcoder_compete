use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize);n]
    }

    // 前日比の出席者増減数を配列にする
    let mut v: Vec<isize> = vec![0; d];
    for i in 0..n {
        let (l, r) = lr[i];
        let li = l - 1;
        let ri = r - 1;
        v[li] += 1;
        if ri != d - 1 {
            v[ri + 1] -= 1;
        }
    }

    // 前日比の出席者増減配列から、累積和配列を作成する
    let mut sum: Vec<usize> = vec![];
    for i in 0..d {
        if i == 0 {
            sum.push(v[0] as usize);
            continue;
        }
        // dbg!(&sum);

        // 下記では usizeキャストが v[i] にかかっており、v[i]が負数となる場合にコンパイルエラーとなる
        // sum.push(sum.last().unwrap() + v[i] as usize);
        sum.push((sum[i - 1] as isize + v[i]) as usize);
    }

    sum.iter().for_each(|val| {
        println!("{}", val);
    })
}
