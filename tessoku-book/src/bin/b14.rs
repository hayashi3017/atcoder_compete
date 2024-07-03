use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        mut a: [usize;n]
    }
    a.sort();
    dbg!(&a);
    // tips: 配列を均等に2分する
    let (a1, a2) = a.split_at(n / 2);
    dbg!(&a1);
    dbg!(&a2);

    
}
