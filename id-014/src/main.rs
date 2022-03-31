use std::collections::HashMap;

fn main() {
    let i = 1000000;
    /*
        let now = std::time::Instant::now();
        let a1 = longs_path_bf(i);
        let t1 = now.elapsed().as_millis();
        println!("bf answer: {}  | time: {}", a1, t1);
    */
    let now = std::time::Instant::now();
    let a2 = longs_path_hm_opt(i);
    let t2 = now.elapsed().as_millis();

    println!("hm answer: {}  | time: {}", a2, t2);
}

fn longs_path_bf(top_starting_number: u128) -> u128 {
    let mut longest_path_length: u128 = 0;
    let mut longest_path_start: u128 = 0;
    for mut x in 1..top_starting_number {
        let mut current_path: Vec<u128> = vec![x];
        let mut current_path_length: u128 = 1;
        loop {
            if x % 2 == 0 {
                x = x / 2;
            } else {
                x = 3 * x + 1;
            }
            current_path_length += 1;
            current_path.push(x);
            if x == 1 {
                break;
            }
        }
        if current_path_length > longest_path_length {
            longest_path_start = *current_path.first().unwrap();
            longest_path_length = current_path_length;
        }
    }
    longest_path_start
}

fn longs_path_hm_opt(top_starting_number: u128) -> u128 {
    // key is the position and value is the steps to 1
    let mut visited_position: HashMap<u128, u128> = HashMap::new();
    let mut longest_path_length: u128 = 0;
    let mut longest_path_start: u128 = 0;
    for mut x in 1..top_starting_number {
        let mut current_path: Vec<u128> = vec![x];
        let mut current_path_length: u128 = 1;
        loop {
            if visited_position.contains_key(&x) {
                current_path_length += visited_position.get(&x).unwrap();
                for (i, pos) in current_path.iter().enumerate() {
                    visited_position.insert(*pos, current_path_length - i as u128);
                }
                break;
            }
            if x % 2 == 0 {
                x = x / 2;
            } else {
                x = 3 * x + 1;
            }

            current_path_length += 1;
            current_path.push(x);
            if x == 1 {
                break;
            }
        }
        if current_path_length > longest_path_length {
            /*
            current_path.iter().take(20).for_each(|x| print!("{} ", x));
            println!(
                " cpl = {} | lpl ={} | cp_len = {}",
                current_path_length,
                longest_path_length,
                current_path.len()
            );
            */
            longest_path_start = *current_path.first().unwrap();
            longest_path_length = current_path_length;
        }
    }
    longest_path_start
}
