fn main() {
    let sum: u128 = (0..)
        .into_iter()
        .take_while(|x| *x < 2000000)
        .filter(|x| is_prime(*x))
        .map(|x| x as u128)
        .sum();
    println!("{}", sum);
}

fn is_prime(i: usize) -> bool {
    if i <= 1 {
        false
    } else if i <= 3 {
        true
    } else if i % 2 == 0 || i % 3 == 0 {
        false
    } else {
        let mut n = 5;
        while n * n <= i {
            if i % n == 0 || i % (n + 2) == 0 {
                return false;
            }
            n += 6;
        }
        return true;
    }
}
