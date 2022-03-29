use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn main() {
    flp3d()
}

fn flp3d() {
    let now = std::time::Instant::now();
    let t = (0..1000)
        .rev()
        .cartesian_product((0..1000).rev())
        .map(|(x1, x2)| x1 * x2 as i32)
        .dedup()
        .filter(|x| {
            match x
                .to_string()
                .chars()
                .zip(x.to_string().chars().rev())
                .fold_while(true, |_, (c1, c2)| {
                    if c1 == c2 {
                        Continue(true)
                    } else {
                        Done(false)
                    }
                }) {
                Continue(t) => t,
                Done(t) => t,
            }
        })
        .max()
        .unwrap();
    let duration = now.elapsed().as_millis();
    println!("iterator sol\nsol is {}\nthis took: {}\n", t, duration);
    let now = std::time::Instant::now();
    let mut current_max = 0;
    'outta: for x1 in (0..1000).rev() {
        for x2 in (0..x1 + 1).rev() {
            if let Continue(_) = ((x1 * x2) as usize)
                .to_string()
                .chars()
                .zip((x1 * x2).to_string().chars().rev())
                .fold_while(true, |_, (c1, c2)| {
                    if c1 == c2 {
                        Continue(true)
                    } else {
                        Done(false)
                    }
                })
            {
                //println!("solution is: {}", x1 * x2);
                if x1 * x2 > current_max {
                    current_max = x1 * x2;
                }
            }
            if current_max > x1 * x1 {
                let duration = now.elapsed().as_millis();
                println!(
                    "forloop v1\nsolution is: {}\nthis took {}",
                    current_max, duration
                );

                break 'outta;
            }
            //println!("{} {} {}", x1, x2, x1 * x2);
        }
    }
}
