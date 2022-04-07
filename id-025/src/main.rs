use num::bigint::BigUint;

fn main() {
    println!("{}", first_1000_dig_fib());
}

fn first_1000_dig_fib() -> usize {
    let mut value_storage: Vec<BigUint> = vec![BigUint::new(vec![1]), BigUint::new(vec![1])];
    for x in 2.. {
        let new_value: BigUint = value_storage[x - 1].clone() + value_storage[x - 2].clone();
        if new_value.to_str_radix(10).len() >= 1000 {
            return x + 1;
        }
        value_storage.push(new_value);
    }
    panic!()
}
