use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut cnt = 0;
    for i in 0..n {
        for j in i + 1..n {
            if (a[i] + a[j]) % 100 == 0 {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}
