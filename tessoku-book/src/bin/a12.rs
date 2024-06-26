use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n]
    }

    // TODO: 未解決。途中。
    let l = 100_000_000usize;
    let mut left = 0;
    let mut right = l / 2;
    while right - left > 1 {
        let diff = right - left;
        let mut sum = 0;
        for i in 0..n {
            sum += diff / a[i];
        }

        if sum < k {
            left = right;
            right *= 2;
        } else if sum == k {
            println!("{}", sum);
            break;
        } else if k < sum {
            right /= 2;
        }
    }
}
