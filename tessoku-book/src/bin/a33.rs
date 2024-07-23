use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    if first_player_win(&a) {
        println!("First");
    } else {
        println!("Second");
    }

    fn first_player_win(a: &[usize]) -> bool {
        let xor = a.iter().fold(0, |acc, x| {
            return acc ^ *x;
        });
        return xor != 0;
    }
}
