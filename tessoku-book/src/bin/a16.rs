use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n-1],
        b: [usize;n-2],
    }

    let mut v: Vec<usize> = vec![0, a[0]];
    // FIXME: v,a,bのindexを揃えた方が考えやすい
    for i in 0..n-2 {
        let min = (v[i + 1] + a[i + 1]).min(v[i] + b[i]);
        v.push(min);
    }

    println!("{}", v.last().unwrap());
}
