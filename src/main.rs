use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    let mut input = input
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .expect("Failed to parse number from input")
        })
        .collect::<Vec<_>>();

    input.push(0);
    input.push(input.iter().max().expect("Data set was empty!") + 3);

    input.sort();

    // Calculate the deltas
    let differences = input
        .iter()
        //.skip(1)
        .zip(input.iter().skip(1))
        .map(|(left, right)| right - left)
        .collect::<Vec<_>>();

    let difference_options = differences
        .iter()
        .unique()
        .map(|unique_option| {
            (
                unique_option,
                differences.iter().filter(|x| *x == unique_option).count(),
            )
        })
        .collect::<Vec<_>>();

    // Multiply our options
    let multiplied = difference_options
        .iter()
        .fold(1, |acc, (_, count)| acc * *count);

    println!("{:?}", multiplied);
}
