use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut a: [usize;n],
        mut b: [usize;n],
    }

    a.sort();
    a.reverse();
    b.sort();
    b.reverse();
    let mut sx = 0usize;
    let mut mx = 0usize;
    let mut sy = 0usize;
    let mut my = 0usize;

    for i in 0..n {
        if sx <= x {
            sx += a[i];
            mx = i + 1;
        }
        if sy <= y {
            sy += b[i];
            my = i + 1;
        }
    }

    println!("{}", mx.min(my));
}
