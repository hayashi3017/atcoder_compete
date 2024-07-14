use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        q: usize,
        x: [usize;q]
    }

    for x in x {
        if prime_check(x) {
            println!("No");
        } else {
            println!("Yes");
        }
    }

    fn prime_check(x: usize) -> bool {
        let l = x.sqrt();
        for i in 2..=l {
            if x % i == 0 {
                return true;
            }
        }
        return false;
    }
}
