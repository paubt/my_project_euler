use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("../data/id-008/input.txt").expect("read in file failed");

    println!("{}", fgp(13, &input));
}

fn fgp(ws: usize, numbers: &str) -> u128 {
    let numbers: Vec<u128> = numbers
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|n| n.to_string().parse::<u128>().unwrap())
        .collect();
    let mut current_product: u128 = numbers.iter().take(ws).product();
    let mut highes_product: u128 = current_product;
    for i in ws..numbers.len() {
        if numbers[i - ws] != 0 {
            current_product = current_product / numbers[i - ws];
            if numbers[i] != 0 {
                current_product = current_product * numbers[i];
            } else {
                current_product = 0;
            }
        } else {
            current_product = numbers.iter().skip(i - ws + 1).take(ws).product();
        }
        if current_product > highes_product {
            highes_product = current_product;
        }
    }
    highes_product
}
