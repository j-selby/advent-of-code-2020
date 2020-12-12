fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    let input = input.lines()
        .map(|line| {
            line.parse::<i64>().expect("Failed to parse number from input")
        })
        .collect::<Vec<_>>();

    for (index, num) in input.iter().enumerate().skip(25) {
        let preamble = &input[index - 25 .. index];
        // Calculate if any preamble pair matches
        let mut found_match = false;

        'main_loop:
        for num_a in preamble {
            for num_b in preamble {
                if num_a + num_b == *num {
                    found_match = true;
                    break 'main_loop;
                }
            }
        }

        if !found_match {
            println!("Bad match: {:?}", num);
            break;
        }
    }

}
