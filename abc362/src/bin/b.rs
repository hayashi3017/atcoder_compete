use proconio::input;

fn main() {
    input! {
        a: (isize, isize),
        b: (isize, isize),
        c: (isize, isize),
    }

    let ab = (a.0 - b.0).abs().pow(2) + (a.1 - b.1).abs().pow(2);
    let bc = (b.0 - c.0).abs().pow(2) + (b.1 - c.1).abs().pow(2);
    let ca = (c.0 - a.0).abs().pow(2) + (c.1 - a.1).abs().pow(2);

    if ca == ab + bc || ab == bc + ca || bc == ca + ab {
        println!("Yes");
    } else {
        println!("No");
    }
}
