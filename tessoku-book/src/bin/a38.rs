use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize, usize, usize);n]
    }

    let mut ans = vec![24usize; d];
    for (l , r, h) in lrh {
        for i in l-1..r {
            ans[i] = ans[i].min(h);
        }
    }

    println!("{}", ans.iter().sum::<usize>());
}
