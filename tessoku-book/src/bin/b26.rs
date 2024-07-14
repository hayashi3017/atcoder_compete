use proconio::input;

//　エラストテネスの篩で解く
// FIXME: n=1000000の時にTLEになっている
fn main() {
    input! {
        n: usize
    }

    let mut p = vec![true; n];
    (2..=n).for_each(|v| {
        if p[v - 1] {
            println!("{}", v);
            (v + 1..=n).for_each(|x| {
                if x % v == 0 {
                    p[x - 1] = false;
                }
            });
        }
    });
}
