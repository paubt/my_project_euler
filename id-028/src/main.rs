fn main() {
    println!("{}", sum_spirl_diag(1001));
}

fn sum_spirl_diag(n: u64) -> u64 {
    let mut sum: u64 = 1;
    let mut last_value = 1;
    for skip_dis in (1..n).step_by(2) {
        // println!("sd={} sum={}", skip_dis, sum);
        for _ in 0..4 {
            last_value = last_value + skip_dis + 1;
            sum += last_value;
            // println!("{}", last_value);
        }
    }
    sum
}
/*
// iternet solution

fn sum_spirl_diag_2(n: u64) -> u64 {
    let mut sum: u64 = 1;

    let last = n * n;
    let mut d = 2;
    let mut k = 0;
    let mut i = 3;
    while i <= last {
        sum += i;
        println!("{}", i);
        k += 1;
        if k % 4 == 0 {
            println!("sum = {}", sum);
            d += 2;
        }
        i += d;
    }
    sum
}
*/
