#![feature(iterator_fold_self)]

use std::collections::BTreeSet;

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    let result : usize = input
        .split("\n\n")
        .map(|x| {
            x.split_whitespace().map(|passenger| passenger.chars().collect::<BTreeSet<char>>())
                .fold_first(|acc, a| acc.intersection(&a).map(|x| *x).collect())
                .expect("Failed to fold elements")
                .len()
        })
        .sum();

    println!("{:?}", result);
}
