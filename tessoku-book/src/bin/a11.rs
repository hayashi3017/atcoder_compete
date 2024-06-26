use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize;n],
    }

    // let ans = a.binary_search(&x).unwrap() + 1;
    // println!("{}", ans);

    let mut l = 0usize;
    let mut r = n;
    let mut ans = 0;

    while l <= r {
        let m = (r + l) / 2;
        if a[m] < x {
            l = m + 1;
            continue;
        }
        if a[m] == x {
            ans = m + 1;
            break;
        }
        if a[m] > x {
            r = m - 1;
            continue;
        }
    }
    println!("{}", ans);
}
