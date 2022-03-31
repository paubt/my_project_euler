use std::collections::HashMap;

fn main() {
    let t = 500;
    let now = std::time::Instant::now();
    let a1 = hdtn(t);
    let d1 = now.elapsed().as_nanos();
    println!("{} {} time: {}", t, a1, d1);
    /*
    let now = std::time::Instant::now();
    let a2 = naiv_sol(t);
    let d2 = now.elapsed().as_nanos();
    println!("{} {} time: {}", t, a2, d2);
    */
}

fn naiv_sol(x: usize) -> usize {
    let mut t = 1;
    let mut a = 1;
    let mut cnt = 0;
    while cnt <= x {
        cnt = 0;
        a += 1;
        t += a;
        for i in 1..t {
            if t % i == 0 {
                cnt += 1;
            }
        }
    }
    t
}

fn prime_factor_count(mut x: usize) -> usize {
    let mut pf: HashMap<usize, usize> = HashMap::new();

    let og_number = x;
    while x % 2 == 0 {
        x /= 2;
        let t = pf.entry(2).or_insert(0);
        *t = *t + 1;
    }

    for i in (3..((og_number as f64).sqrt().ceil() as usize)).step_by(2) {
        while x % i == 0 {
            x /= i;
            let t = pf.entry(i).or_insert(0);
            *t = *t + 1;
        }
    }

    if x > 2 {
        let t = pf.entry(x).or_insert(0);
        *t = *t + 1;
        //println!("lel x={}", x);
    }
    pf.into_values().fold(1, |acc, x| acc * (x + 1))
}

fn hdtn(nod: usize) -> usize {
    let mut tn = 0;
    for add in 1.. {
        tn += add;
        if prime_factor_count(tn) > nod {
            break;
        }
    }
    tn
}
