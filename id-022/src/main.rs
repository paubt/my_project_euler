use std::fs;

fn main() {
    let names = fs::read_to_string("../data/id-022/input.txt").expect("file read in failed");
    println!("{}", score_name(names));
}

fn score_name(mut names: String) -> u128 {
    names = names.replace("\"", "");
    let mut names: Vec<&str> = names.lines().nth(0).unwrap().split(",").collect();
    names.sort_by(|a, b| a.cmp(b));
    names
        .iter()
        .map(|n| n.chars().fold(0, |acc, c| acc + (c as usize - 64)))
        .enumerate()
        .fold(0, |acc, (i, n)| acc + (n * (i + 1)) as u128)
}
