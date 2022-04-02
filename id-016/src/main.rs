use num::bigint::BigUint;
fn main() {
    let t = BigUint::new(vec![2])
        .pow(1000)
        .to_str_radix(10)
        .chars()
        .fold(0, |acc, c| acc + c.to_string().parse::<u128>().unwrap());

    println!("{}", t);
}
