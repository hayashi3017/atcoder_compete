use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String;n]
    }
    s.reverse();
    let m = s.iter().map(|v| v.len()).max().unwrap();
    // dbg!(m);
    let ps: Vec<_> = s
        .iter()
        .map(|v| format!("{:width$}", v, width = m).replace(" ", "*"))
        .collect();
    // dbg!(&ps);

    for i in 0..m {
        let mut b = String::new();
        for v in &ps {
            let t = v.chars().nth(i).unwrap().to_string();
            b = [b, t].concat();
        }
        while b.ends_with("*") {
            b.pop();
        }
        println!("{}", b);
    }
}
