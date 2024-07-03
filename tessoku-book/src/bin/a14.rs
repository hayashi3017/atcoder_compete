use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n],
        b: [usize;n],
        c: [usize;n],
        d: [usize;n],
    }

    let mut v1 = vec![];
    let mut v2 = vec![];
    for i in 0..n {
        for j in 0..n {
            v1.push(a[i] + b[j]);
            v2.push(c[i] + d[j]);
        }
    }
    // dbg!(&v1);
    // dbg!(&v2);
    v2.sort();

    let mut ans = false;
    'outer: for i in 0..v1.len() {
        // TODO: 何かが間違っている、二分探索
        // 参考　https://atcoder.jp/contests/tessoku-book/submissions?f.LanguageName=Rust&f.Status=AC&f.Task=tessoku_book_n&f.User=&orderBy=source_length
        // 誤解答 https://atcoder.jp/contests/tessoku-book/submissions/55157325
        // ケース　https://www.dropbox.com/sh/nx3tnilzqz7df8a/AAA_QLZ_jho5Jx_mRm9sg5hSa/tessoku-book/A14?dl=0
        // let mut l = 0usize;
        // let mut r = v1.len();
        // // eprintln!("======== {}", i);
        // while l < r {
        //     let m = (l + r) / 2;
        //     // eprintln!("-------");
        //     // dbg!(l);
        //     // dbg!(r);
        //     // dbg!(m);
        //     if v1[i] < (k - v2[m]) {
        //         l = m + 1;
        //     }
        //     if v1[i] == (k - v2[m]) {
        //         ans = true;
        //         // tips: 二重のループを抜けるにはラベルを使う
        //         break 'outer;
        //     }
        //     if v1[i] > (k - v2[m]) {
        //         r = m;
        //     }
        // }

        if v2.binary_search(&(k - v1[i])).is_ok() {
            ans = true;
            break;
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
