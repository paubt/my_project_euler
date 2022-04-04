use num::bigint::BigUint;

fn main() {
    println!("{}", fac_len(100));
}

fn fac_len(n: u128) -> u128 {
    (2..=n)
        .fold(BigUint::new(vec![1]), |acc, i| acc * i)
        .to_str_radix(10)
        .chars()
        .fold(0, |acc, c| acc + c.to_string().parse::<u128>().unwrap())
}
