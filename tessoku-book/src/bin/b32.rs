use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize;k]
    }

    a.sort();

    if first_player_win(n, &a) {
        println!("First")
    } else {
        println!("Second")
    }

    fn first_player_win(n: usize, a: &[usize]) -> bool {
        if n < a[0] {
            return false;
        }
        let mut res = false;
        for v in a {
            if n >= *v {
                res |= !first_player_win(n - v, a)
            }
        }
        res
    }
}
