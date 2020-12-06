use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    let result : usize = input
        .split("\n\n")
        .map(|x| x.chars().filter(|x| x.is_alphabetic()).unique().count())
        .sum();

    println!("{:?}", result);
}
