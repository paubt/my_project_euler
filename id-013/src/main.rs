use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("../data/id-013/input.txt").expect("read in file failed");

    println!("{}", sum_td(&input));
}

fn sum_td(s: &str) -> String {
    let t = s
        .lines()
        .fold(0, |acc, x| {
            acc + x
                .chars()
                .take(11)
                .collect::<String>()
                .parse::<u128>()
                .unwrap()
        })
        .to_string()
        .chars()
        .take(10)
        .collect::<String>();
    t
}
