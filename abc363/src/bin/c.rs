use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: String,
    }

    fn nijo(n: usize) -> usize {
        let mut c = 1;
        for i in 1..=n {
            c *= i;
        }
        c
    }

    fn init_groups(s: &str) -> Vec<Vec<char>> {
        let mut groups: Vec<Vec<char>> = Vec::new();
        let mut p = s.chars().sorted().peekable();

        while let Some(c) = p.next() {
            let mut group = vec![c];
            while let Some(v) = p.peek() {
                if c == *v {
                    group.push(c);
                    p.next();
                } else {
                    break;
                }
            }
            groups.push(group);
        }
        groups
    }

    // let mut group = init_groups(&s);
    // dbg!(&group);
    // let num = group.iter().fold(1, |acc, x| acc * nijo(x.len()));
    // dbg!(num);
    // let mut ans = nijo(n) / num;
    // dbg!(ans);

    let test = s.chars().collect::<Vec<char>>();
    let test2 = test.iter().permutations(s.len());
    for v in test2 {
        dbg!(v);
    }

    // 検証のために、文字の全ケースを持っておく必要があるのか？

    for i in 1..=(n - k) {
        for j in 1..=k {}
    }
}
