fn main() {
    println!("{}", cumulate_sum_paths(20));
}

fn cumulate_sum_paths(grid_size: usize) -> u128 {
    let mut a: Vec<Vec<u128>> = vec![vec![0; grid_size + 1]; grid_size + 1];
    // fill top row with a zero at start position
    // and than all ones to the left
    a.iter_mut()
        .next()
        .unwrap()
        .iter_mut()
        .skip(1)
        .for_each(|e| *e = 1);

    for r in 1..grid_size + 1 {
        a[r][r] = a[r - 1][r] * 2;
        for c in r + 1..grid_size + 1 {
            a[r][c] = a[r][c - 1] + a[r - 1][c];
        }
    }
    a[grid_size][grid_size]
}
