fn main() {
    println!("hpf {}", gen_prim_list(100));
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

fn gen_prim_list(max: usize) -> usize {
    let ul = (max as f64).sqrt().ceil() as usize;
    (0..ul)
        .into_iter()
        .rev()
        .filter(|x| is_prime(*x) && max % x == 0)
        .next()
        .unwrap()
}
