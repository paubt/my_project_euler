use std::collections::VecDeque;

// just sum the the 5ish pairs listed on wikipedia

fn main() {
    println!("{}", sum_amicable_pairs(gen_div_sum_map(10000)));
}

fn sum_amicable_pairs(m: Vec<usize>) -> usize {
    let mut sum: usize = 0;
    for i in 0..m.len() {
        if m[i] > i && m[i] < m.len() && m[m[i]] == i {
            // println!("pair: {} {}", i, m[i]);
            sum += i + m[i];
        }
    }
    sum
}

fn gen_div_sum_map(top: usize) -> Vec<usize> {
    (0..=top).fold(Vec::new(), |mut acc, x| {
        acc.push(
            (1..x)
                .zip(vec![x; x])
                .filter(|(i, t)| t % i == 0)
                .map(|(i, _)| i)
                .sum(),
        );
        acc
    })
}
