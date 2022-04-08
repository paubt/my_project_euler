use itertools::Itertools;
use primes;

use std::cmp::Ordering;

fn main() {
    println!("{}", cons_primes_prod_a_b());
}

fn quad_prim_chain(a: i32, b: i32) -> usize {
    (0..)
        .map(|x| x * x + a * x + b)
        .take_while(|x| x.is_positive() && primes::is_prime(*x as u64))
        .count()
}

fn cons_primes_prod_a_b() -> i32 {
    let max_a = 999;
    let max_b = 1000;
    let (length, product) = (-max_a..=max_a)
        .into_iter()
        .cartesian_product((-max_b..=max_b).into_iter())
        .fold((0, 0), |(length, product), (a, b)| {
            if quad_prim_chain(a, b) > length {
                println!("len ={} , {}*{}={}", quad_prim_chain(a, b), a, b, a * b);
                (quad_prim_chain(a, b), a * b)
            } else {
                (length, product)
            }
        });

    //a_b_list.sort_by(|b, a| (a.0.abs() * a.1.abs()).cmp(&(b.0.abs() * b.1.abs())));
    product
}
