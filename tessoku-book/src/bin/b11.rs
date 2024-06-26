use proconio::input;

// TODO: 未解決。binary_search_by()の挙動はRustバージョンによって異なる様子。挙動としては検索に合致する要素が複数ある場合にErr()を返している様子。合致するならOk()を返してもらいたいが・・精査必要
fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
        q: usize,
        x: [usize;q]
    }
    a.sort();
    for i in x {
        let s = a.binary_search_by(|v| v.cmp(&i));
        match s {
            Ok(v) => {
                println!("{}", v - 1);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
