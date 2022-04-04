use std::collections::HashSet;
fn main() {
    println!("{}", non_abundant_sums());
}

fn non_abundant_sums() -> usize {
    const TOP: usize = 28123;

    let abundant_numbers: Vec<(usize, usize)> = (1..=TOP)
        .map(|n| {
            let mut s = 0;
            for t in 1..n {
                if n % t == 0 {
                    s += t;
                }
            }
            s
        })
        .zip(1 as usize..)
        .filter(|(x, i)| x > i)
        .collect();

    let mut hs: HashSet<usize> = HashSet::new();
    for i in 0..abundant_numbers.len() {
        for j in i..abundant_numbers.len() {
            hs.insert(abundant_numbers[i].1 + abundant_numbers[j].1);
        }
    }

    let mut final_sum = 0;
    for i in 0..TOP {
        if let None = hs.get(&i) {
            final_sum += i;
        }
    }

    final_sum
}
