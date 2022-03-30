fn main() {
    println!("{}", ssd());
}

fn suosq(max: u128) -> u128 {
    let mut sum: u128 = 0;
    for x in 1..max + 1 {
        sum += x * x;
    }
    sum
}

fn sqosu(max: u128) -> u128 {
    let sum: u128 = (0..max + 1).sum();
    sum * sum
}

fn ssd() -> u128 {
    let max: u128 = 100;
    println!("{}  {}", sqosu(max), suosq(max));
    sqosu(max) - suosq(max)
}
