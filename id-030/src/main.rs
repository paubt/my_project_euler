use num::pow;

fn main() {
    println!("{}", dfp());
}

fn dfp() -> u32 {
    let t: Vec<u32> = (1..1000000)
        .map(|x: i32| x.to_string())
        .filter(|x| x.len() > 1)
        .fold(Vec::new(), |mut acc, x| {
            let n: u32 = x.chars().map(|c| pow(c.to_digit(10).unwrap(), 5)).sum();
            if n == x.parse::<u32>().ok().unwrap() {
                acc.push(x.parse::<u32>().ok().unwrap());
                acc
            } else {
                acc
            }
        });
    t.iter().sum::<u32>()
}
