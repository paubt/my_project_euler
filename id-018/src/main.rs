use std::fs;

fn main() {
    let triangle = fs::read_to_string("../data/id-018/input.txt").ok().unwrap();
    println!("{}", max_sum_path(&triangle));
}

fn max_sum_path(triangle: &str) -> u128 {
    // transform using vectors
    // use a zero as padding front and back
    // this eliminates the need for a special case when checking the first
    // and last element
    let mut triangle: Vec<Vec<u128>> = triangle
        .lines()
        .map(|l| {
            let mut t = l.split_whitespace().fold(vec![0], |mut acc, e| {
                acc.push(e.parse::<u128>().unwrap());
                acc
            });
            t.push(0);
            t
        })
        .collect();
    // add the larger number from the 2 path above where the path can come from
    for deepth in 1..triangle.len() {
        for i in 1..triangle[deepth].len() - 1 {
            if triangle[deepth - 1][i - 1] > triangle[deepth - 1][i] {
                triangle[deepth][i] += triangle[deepth - 1][i - 1];
            } else {
                triangle[deepth][i] += triangle[deepth - 1][i];
            }
        }
    }
    // find the largest element in the last row (deepest)
    *triangle.last().unwrap().iter().max().unwrap()
}
