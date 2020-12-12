#![feature(option_expect_none)]

use std::collections::HashMap;

fn find_options(current_jolts : i64, input: &[i64], cache: &mut HashMap<i64, i64>) -> i64 {
    if input.len() == 1 {
        return 1;
    }

    input.iter()
        .enumerate()
        .filter(|(_index, adapter_jolts)| {
            let diff = *adapter_jolts - current_jolts;
            diff <= 3 && diff > 0
        })
        .map(|(index, adapter_jolts)| {
            match cache.get(adapter_jolts) {
                Some(x) => {
                    *x
                }
                None => {
                    let new_value = find_options(*adapter_jolts, &input[index + 1 ..], cache);
                    cache.insert(*adapter_jolts, new_value)
                        .expect_none("Cache should have been empty");
                    new_value
                }
            }
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    let mut input = input
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .expect("Failed to parse number from input")
        })
        .collect::<Vec<_>>();

    input.push(input.iter().max().expect("Data set was empty!") + 3);

    input.sort();

    let mut map = HashMap::new();

    println!("options: {}", find_options(0, &input, &mut map));
}
