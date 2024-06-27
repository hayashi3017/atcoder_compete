use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n]
    }

    let l = 100_000_000usize;
    let mut left = 1;
    let mut right = l;
    while left < right {
        let mid = (left + right) / 2;
        let mut sum = 0;
        for v in &a {
            sum += mid / v;
        }

        if sum < k {
            left = mid + 1;
        } else if k <= sum {
            right = mid;
        }
        // 下記ではNG.
        // left = rightとなり left = midの無限ループとなる
        // if sum <= k {
        //     left = mid;
        // } else if k < sum {
        //     right = mid - 1;
        // }
    }

    println!("{}", left);
}
