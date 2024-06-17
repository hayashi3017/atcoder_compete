use proconio::input;

fn main() {
    input! {
        N: usize,
        Y: usize,
    }

    let mut x: isize = -1;
    let mut y: isize = -1;
    let mut z: isize = -1;

    for i in 0..N + 1 {
        for j in 0..N + 1 {
            if N < i + j {
                continue;
            }
            let k = N - (i + j);
            if (10000 * i) + (5000 * j) + (1000 * k) == Y {
                x = i as isize;
                y = j as isize;
                z = k as isize;
                break;
            }
        }
    }

    println!("{} {} {}", x, y, z);
}
