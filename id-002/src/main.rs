fn main() {
    println!("{}", fbeup());
}

fn fbeup() -> usize {
    let mut l1: usize = 2;
    let mut l2: usize = 1;
    let mut sum: usize = 2;
    loop {
        let new: usize = l1 + l2;
        if new > 4000000 {
            break;
        } else if new % 2 == 0 {
            sum += new;
        }
        l2 = l1;
        l1 = new;
    }
    sum
}
