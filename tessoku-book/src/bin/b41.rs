use proconio::input;

fn main() {
    input! {
        lx: usize,
        ly: usize,
    }

    let mut sm = 1;
    let mut bg = 1;
    let mut h = vec![];

    if lx < ly {
        while sm != lx {
            sm += bg;
            h.push((sm, bg));
        }
        while bg != ly {
            bg += sm;
            h.push((sm, bg));
        }
    } else {
        while bg != ly {
            bg += sm;
            h.push((sm, bg));
        }
        while sm != lx {
            sm += bg;
            h.push((sm, bg));
        }
    }
    println!("{}", h.len());
    (0..h.len()).for_each(|i| println!("{} {}", h[i].0, h[i].1));
}
