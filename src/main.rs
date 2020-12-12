fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    let input = input
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .expect("Failed to parse number from input")
        })
        .collect::<Vec<_>>();

    let mut bad_num = None;

    for (index, num) in input.iter().enumerate().skip(25) {
        let preamble = &input[index - 25..index];
        // Calculate if any preamble pair matches
        let mut found_match = false;

        'main_loop: for num_a in preamble {
            for num_b in preamble {
                if num_a + num_b == *num {
                    found_match = true;
                    break 'main_loop;
                }
            }
        }

        if !found_match {
            println!("Bad match: {:?}", num);
            bad_num = Some((index, *num));
            break;
        }
    }

    let (bad_index, bad_num) = bad_num.expect("No match in the data set");

    // Find the weakness
    for starting_range in 0..bad_index {
        // See if we get a sum which matches
        let mut sum = 0;
        for ending_range in starting_range..bad_index {
            sum += input[ending_range];

            if sum == bad_num {
                println!(
                    "Range: {} ({}) .. {} ({})",
                    starting_range, input[starting_range], ending_range, input[ending_range]
                );

                // Find the smallest number in this range
                let min = input[starting_range..=ending_range]
                    .iter()
                    .min()
                    .expect("No numbers in range");
                let max = input[starting_range..=ending_range]
                    .iter()
                    .max()
                    .expect("No numbers in range");

                println!("Min/max: {} + {} = {}", min, max, min + max);
            }
        }
    }
}
