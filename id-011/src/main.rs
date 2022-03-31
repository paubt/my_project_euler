use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("../data/id-011/input.txt").expect("read in file failed");

    println!("{}", ft4ig(&input));
}

fn ft4ig(input: &str) -> u32 {
    let window_size = 4;
    let a: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    let mut hp: u32 = 0;
    // up
    for r in 0..(a.len() - window_size + 1) {
        for c in 0..a.first().unwrap().len() {
            let t = a[r][c] * a[r + 1][c] * a[r + 1][c] * a[r + 3][c];
            if t > hp {
                hp = t;
            }
        }
    }
    // left
    for r in 0..a.len() {
        for c in 0..a.iter().nth(r).unwrap().len() - window_size + 1 {
            let t = a[r][c] * a[r][c + 1] * a[r][c + 2] * a[r][c + 3];
            if t > hp {
                hp = t;
            }
        }
    }
    // diagonally nw-se
    for r in 0..a.len() - window_size + 1 {
        for c in 0..a.iter().nth(r).unwrap().len() - window_size + 1 {
            let t = a[r][c] * a[r + 1][c + 1] * a[r + 2][c + 2] * a[r + 3][c + 3];
            if t > hp {
                hp = t;
            }
        }
    }
    // diagonally sw-ne
    for r in window_size - 1..a.len() {
        for c in 0..a.iter().nth(r).unwrap().len() - window_size + 1 {
            let t = a[r][c] * a[r - 1][c + 1] * a[r - 2][c + 2] * a[r - 3][c + 3];
            if t > hp {
                hp = t;
            }
        }
    }
    hp
}
