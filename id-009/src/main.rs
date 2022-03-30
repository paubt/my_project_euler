fn main() {
    for m in 2.. {
        for n in 1..m {
            if m * (n + m) == 500 {
                let m: u128 = m;
                let n: u128 = n;
                let awnser = (m * m - n * n) * (2 * m * n) * (m * m + n * n);
                println!("{}", awnser);
                return;
            }
        }
    }
}
