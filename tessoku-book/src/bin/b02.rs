use proconio::input;

const NUM: usize = 100;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let d = get_divisor(NUM);

    let mut exists = false;
    for n in a..b {
        if d.contains(&n) {
            exists = true;
            break;
        }
    }

    if exists {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn get_divisor(number: usize) -> Vec<usize> {
    let mut c = Vec::new();
    for n in 1..number {
        if number % n == 0 {
            c.push(n);
        }
    }
    c
}
