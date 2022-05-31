use std::collections::{LinkedList, HashSet};

fn concat(vec: &[u8]) -> u32
{
    vec.iter().fold(0, |acc, elem| acc * 10 + (elem.to_owned() as u32))
}

// return count and result of the multiplication
fn check_combo(v: Vec<u8>) -> Option<u32>
{
    // last 4 digits
    let target_result = concat(&v.as_slice()[5..]);
    // check all 4 possible positions for the multi sign
    let c = (1..5).fold(0,|acc, p|
        if concat(&v.as_slice()[..p]) * concat(&v.as_slice()[p..5]) == target_result
        {
            acc + 1
        }
        else
        {
            acc
        }
    );
    if c > 0
    {
        Some(target_result)
    }
    else
    {
        None
    }
}

fn main()
{
    let MAX: u8 = 10;
    // create all possible combinations of the 9 digits
    let mut possible_combos: LinkedList<Vec<u8>> = (2..MAX)
        .fold(
            {
                    let mut t = LinkedList::new();
                    t.push_back(vec![1]);
                    t
            },
            |acc, n| {
                let npc  = acc
                    .iter()
                    .fold(LinkedList::new(), |mut acc, v| {
                        let mut kemp: LinkedList<Vec<u8>> = (0..n).map(|idx| {
                                let mut t = v.clone();
                                t.insert(idx.into(), n);
                                t
                            })
                            .collect();
                        acc.append(&mut kemp);
                        acc
                    });
                npc });
    // transform all possible to set of all obtainable
    let sum_all_legal_combos: u32 = possible_combos
        .iter()
        .fold(HashSet::new(), |mut acc, combo|
            match check_combo(combo.to_vec()) {
                Some(result) => {
                    acc.insert(result);
                    acc
                }
                None => acc
            })
        .iter()
        .sum();
    println!("sum: {}", sum_all_legal_combos);
}
