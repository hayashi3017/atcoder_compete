use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize;n]
    }

    a.sort();

    let mut count = k;
    while count > 0 {
        count -= 1;

        let c1 = a.last().unwrap() - a[1];
        let c2 = a[a.len() - 2] - a[0];
        // c1 == c2の場合にどちらを削除するかによって解が変わってしまう
        // 深さ優先探索が必要？
        if c1 < c2 {
            a.remove(0);
        } else {
            a.remove(a.len() - 1);
        }
        // dbg!(count);
        // dbg!(&a);
    }

    println!("{}", a.last().unwrap() - a[0])
}
