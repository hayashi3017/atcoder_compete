use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
    }

    let v = a.iter().enumerate();
    let mut exists = false;

    // error: use close for A in `for x in A`
    // https://doc.rust-lang.org/error_codes/E0277.html
    for (i, id) in v.clone() {
        for (j, jd) in v.clone() {
            if i == j {
                continue;
            }
            for (k, kd) in v.clone() {
                if i == k || j == k {
                    continue;
                }

                if id + jd + kd == 1000 {
                    exists = true;
                    break;
                }
            }
        }
    }

    if exists {
        println!("Yes");
    } else {
        println!("No");
    }
}
