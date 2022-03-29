fn main() {
    println!("{}", fm1(1000));
}

fn fm1(mut upper_bound: usize) -> usize {
    let mut sum: usize = 0;
    while upper_bound > 0 {
        upper_bound -= 1;
        if upper_bound % 5 == 0 || upper_bound % 3 == 0 {
            // println!("{}", upper_bound);
            sum += upper_bound;
        }
    }
    sum
}
