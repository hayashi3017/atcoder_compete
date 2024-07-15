use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(char, isize);n]
    }

    let mut s = 0;
    const BASE_NUM: isize = 10000;
    for (t, a) in ta {
        if t == '+' {
            s += a;
        } else if t == '-' {
            s -= a;
            if s < 0 {
                s += BASE_NUM;
            }
        } else if t == '*' {
            s *= a;
        }

        s = s % BASE_NUM;
        println!("{}", s);
    }
}
