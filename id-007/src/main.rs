fn main() {
    println!("{}", iest_prime(10001));
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

fn iest_prime(i: u128) -> usize {
    let mut prime_count: u128 = 0;
    for x in 0.. {
        if is_prime(x) {
            prime_count += 1;
            if prime_count == i {
                return x;
            }
        }
    }
    panic!();
}
