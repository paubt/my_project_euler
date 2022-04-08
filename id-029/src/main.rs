use itertools::Itertools;
use num::BigUint;

fn main() {
    println!("{}", dis_pow(100));
}

fn dis_pow(top: usize) -> usize {
    let mut t = (2..=top)
        .cartesian_product(2..=top)
        .map(|(a, b)| BigUint::new(vec![a as u32]).pow(b as u32))
        .collect::<Vec<BigUint>>();
    t.sort();
    t.dedup();
    t.len()
}
