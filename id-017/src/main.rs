use itertools::Itertools;

fn main() {
    let mut s = 0;
    for x in 1..1001 {
        s += reverse_transcribe(x)
            .split_whitespace()
            .fold(0, |acc, ss| acc + ss.len());
    }

    println!("{}", s);
}

fn reverse_transcribe(n: u32) -> String {
    let ones = vec![
        "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
    ];
    let teens = vec![
        "Ten",
        "Eleven",
        "Twelve",
        "Thirteen",
        "Fourteen",
        "Fifteen",
        "Sixteen",
        "Seventeen",
        "Eighteen",
        "Nineteen",
    ];

    let tens = vec![
        "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
    ];

    let thousands = vec!["", "Thousand", "Million", "Billion"];
    let temp = tens
        .iter()
        .cartesian_product(ones.iter())
        .map(|(t, o)| format!("{} {}", t, o))
        .collect::<Vec<String>>();

    let temp2: Vec<&str> = temp.iter().map(|x| x.as_str()).collect();

    let unit: Vec<&str> = ones
        .into_iter()
        .chain(teens.into_iter())
        .chain(temp2.into_iter())
        .collect();

    let ones = vec![
        "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
    ];
    let mut number: u32 = n;
    let mut z = 0;
    let mut output: String = String::new();
    loop {
        if number == 0 {
            break;
        }
        let rest = number % 1000;
        number = number / 1000;
        if rest > 0 {
            let u = rest % 100;
            let h = rest / 100;
            let mut words = format!("{} {}", unit[u as usize], thousands[z]);
            if h > 0 {
                if u > 0 {
                    words = format!("{} Hundred and {}", ones[h as usize], words);
                } else {
                    words = format!("{} Hundred {}", ones[h as usize], words);
                }
            }

            output = format!("{} {}", words, output);
        }
        z += 1;
    }
    output
}
