use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [isize;n],
        bk: [(isize, usize);q],
    }

    a.sort();
    for (b, mut k) in bk {
        let mut ac = a.clone();
        let mut ans = vec![];
        while k > 0 {
            let mut i = 0;
            let l = ac.lower_bound(&b);
            dbg!(k);
            dbg!(l);
            k -= 1;
            let bd = (b - ac[l]).abs();
            let hbd = (b - ac[l + 1]).abs();
            if l + 1 <= n - 1 {
                if bd < hbd {
                    i = l;
                } else {
                    i = l + 1;
                }
            } else {
                i = l;
            }
            ac.remove(i);
            ans[k] = i;
        }
        ans.reverse();
        println!("{}", (b - a[ans[k]]).abs());
    }
}
