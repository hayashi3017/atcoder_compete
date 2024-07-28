use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut sij: (usize, usize),
        cij: [String;h],
        x: String,
    }

    let grid: Vec<Vec<char>> = cij.iter().map(|v| v.chars().collect()).collect();

    for v in x.chars() {
        if v == 'L' {
            let (i, j) = sij;
            if j - 1 > 0 && grid[i - 1][j - 2] == '.' {
                sij = (i, j - 1);
            }
            continue;
        }
        if v == 'R' {
            let (i, j) = sij;
            if j + 1 <= w && grid[i - 1][j] == '.' {
                sij = (i, j + 1);
            }
            continue;
        }
        if v == 'U' {
            let (i, j) = sij;
            if i - 1 > 0 && grid[i - 2][j - 1] == '.' {
                sij = (i - 1, j);
            }
            continue;
        }
        if v == 'D' {
            let (i, j) = sij;
            if i + 1 <= h && grid[i][j - 1] == '.' {
                sij = (i + 1, j);
            }
            continue;
        }
    }

    println!("{} {}", sij.0, sij.1);
}
