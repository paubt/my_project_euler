use big_rational_str::*;
use num::bigint::{BigInt, Sign};
use num::rational::BigRational;

fn largest_recurring_cycle_2(r: BigRational) -> String {
    let t = big_rational_to_string(r);

    let t = match t.split(|x| x == '(' || x == ')').nth(1) {
        None => "",
        Some(x) => x,
    };
    t.to_string()
}

fn main() {
    println!("{}", max_len_rcp_cyc(1000));
}

fn max_len_rcp_cyc(top: usize) -> usize {
    let mut max_len = 0;
    let mut max_d = 0;
    for d in 2..=top {
        let rc = largest_recurring_cycle_2(BigRational::new(
            BigInt::new(Sign::Plus, vec![1]),
            BigInt::new(Sign::Plus, vec![d as u32]),
        ));
        if rc.len() > max_len {
            max_len = rc.len();
            max_d = d;
            //println!("{}  {} ", rc, max_len);
        }
    }
    max_d
}

fn largest_recurring_cycle(n: f64) -> String {
    //println!("{}", n);
    //
    let mut best_solution_cycle: String = String::from("".to_string());
    // transform
    let t = n.to_string();
    let n: Vec<char> = t.split(".").nth(1).unwrap().chars().collect();

    // go for each
    let mut last_chars = 0;
    for x in 0..n.len() {
        //println!("best solutution: {}", best_solution_cycle);
        // find the points where this char allready ocured
        let i_c: Vec<(usize, char)> = n
            .iter()
            .cloned()
            .enumerate()
            .take(x)
            .filter(|(_, c)| *c == n[x])
            .collect();
        // check if this matches as recurring pattern
        'cm: for (i, c) in i_c {
            //println!("{}  {}", i, c);
            let mut found: bool = true;
            for to_end in x..n.len() - 1 {
                if n[to_end - (x - i)] != n[to_end] {
                    found = false;
                    break;
                }
            }
            if found == true {
                //println!("found == true x={} and i={}", x, i);
            }
            if found == true && (x - i) > best_solution_cycle.len() {
                if best_solution_cycle.is_empty() {
                    best_solution_cycle = n.iter().skip(i).take(x - i).collect();
                } else {
                    // check that this is not a multiple from the previous one
                    // perhaps here one should use a hashmap with ALL privious recorded
                    // cycles and check against all
                    let new_solution: String = n.iter().skip(i).take(x - i).collect();
                    //println!("new solution {}  x={} i={}", new_solution, x, i);
                    for x in 2.. {
                        let mut t = String::from(best_solution_cycle.clone());
                        for _ in 0..(x - 2) {
                            t.push_str(&best_solution_cycle.clone());
                            //println!("long clone {}", &t);
                        }
                        if new_solution == t {
                            //println!("({}) == ({})", new_solution, t);
                            continue 'cm;
                        }
                        if t.contains(&best_solution_cycle) {
                            continue 'cm;
                        }

                        if t.len() > n.len() {
                            //println!("break;");
                            break;
                        }
                    }
                    //println!("lel");
                    best_solution_cycle = n.iter().skip(i).take(x - i).collect();
                }
            }
        }
    }
    best_solution_cycle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lrc_1() {
        assert_eq!(
            largest_recurring_cycle(1.0_f64 / 137.0_f64),
            "00729927".to_string()
        );
    }

    #[test]
    fn lrc_3() {
        assert_eq!(
            largest_recurring_cycle(1.0_f64 / 14.0_f64),
            "714285".to_string()
        );
    }
    #[test]
    fn lrc_2() {
        assert_eq!(largest_recurring_cycle(1.0_f64 / 6.0_f64), "6".to_string());
    }
}
