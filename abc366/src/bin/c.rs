use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut m = vec![0; 1000_000];
    let mut cnt = 0;
    for _ in 0..q {
        input! {
            query: usize
        }

        if query == 1 {
            input! {
                x: usize,
            }
            m[x - 1] += 1;
            if m[x - 1] == 1 {
                cnt += 1;
            }
        } else if query == 2 {
            input! {
                x: usize,
            }
            m[x - 1] -= 1;
            if m[x - 1] == 0 {
                cnt -= 1;
            }
        } else if query == 3 {
            println!("{}", cnt);
        }
    }
}
