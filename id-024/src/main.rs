fn main() {
    match lex_prmt(
        vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
        1000000,
    ) {
        Ok(lp) => lp.iter().for_each(|c| print!("{}", c)),
        Err(e) => println!("Error: {}", e),
    }
    println!();
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn lex_prmt(mut input: Vec<char>, mut target_number: usize) -> Result<Vec<char>, String> {
    // first sort the vector lexografical
    input.sort();

    let mut output_vec: Vec<char> = Vec::new();
    while input.len() > 1 {
        let pp_dl_m1 = factorial(input.len() - 1);
        let node_in_range = (target_number - 1) / pp_dl_m1;

        target_number = target_number - node_in_range * pp_dl_m1;
        if node_in_range >= input.len() {
            return Err("target_number not in the permutation range".to_string());
        }
        output_vec.push(input.remove(node_in_range));
    }
    // add the last element
    output_vec.append(&mut input);
    Ok(output_vec)
}
