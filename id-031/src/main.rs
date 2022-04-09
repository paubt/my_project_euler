fn main() {
    println!("{}", coin_sums(200));
}

fn coin_sums(target_amount: usize) -> usize {
    // intit vec
    let mut grid_2d: Vec<Vec<usize>> = vec![vec![1; target_amount]];
    // for each coin
    for coin in [2, 5, 10, 20, 50, 100, 200] {
        if coin > target_amount {
            break;
        }
        // generate new row
        let mut new_row: Vec<usize> = grid_2d.last().unwrap().clone();
        // first is copy from above
        new_row[coin - 1] = new_row[coin - 1] + 1;
        //the rest are sum of previous and above value
        // adding one if a new coin would fit
        for x in coin..target_amount {
            new_row[x] = new_row[x] + new_row[x - coin];
        }
        grid_2d.push(new_row);
    }
    *grid_2d.last().unwrap().last().unwrap()
}
