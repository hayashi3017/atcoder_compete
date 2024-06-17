use proconio::input;

fn main() {
    input! {
        s: String
    }

    let a: Vec<&str> = s.split("2017").collect();
    println!("2018{}", a[1]);
}
