use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,

        d: isize,
        e: isize,
        f: isize,

        g: isize,
        h: isize,
        i: isize,

        j: isize,
        k: isize,
        l: isize,
    }

    // let x1 = d - a;
    // let y1 = e - b;
    // let z1 = f - c;
    // let x2 = j - g;
    // let y2 = k - h;
    // let z2 = l - i;

    let p = (d - g, e - h, f - i);
    let p2 = (g - d, h - e, i - f);
    // dbg!(p);
    if p.0 > 0 && p.1 > 0 && p.2 > 0 || p2.0 > 0 && p2.1 > 0 && p2.2 > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
