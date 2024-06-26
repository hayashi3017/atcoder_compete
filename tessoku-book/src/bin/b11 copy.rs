use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        mut a: [usize],
        x: [usize],
    }
    a.sort();
    for xi in x {
        // tips: ある値より小さくない値を持つ最初の要素のindexを返す
        // 意味としては、lower_boundなので下限を示すっぽい？
        // upper_boundは、ある値より大きい値を持つ最初の要素のindexを返す
        // https://docs.rs/superslice/latest/superslice/trait.Ext.html#tymethod.lower_bound
        // この問題では、xより小さい要素数を求めており、xより小さくない値を持つ最初の要素のindex がそのまま解となる。一つ前の要素はxより小さい値であり要素数はindex+1なので。
        println!("{}", a.lower_bound(&xi));
    }
}
