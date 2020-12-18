use std::collections::HashMap;

fn main() {
    let input = "1,0,15,2,10,13"
        .split(",")
        .map(|x| x.parse().expect("Failed to parse"))
        .collect::<Vec<i64>>();

    // (number, turn)
    let mut last_spoken_turns = HashMap::new();

    let mut last_spoken = 0;
    let mut last_spoken_turn = None;

    let mut turn = 1i64;

    for num in input {
        last_spoken_turns.insert(num, turn);

        // Iterate
        last_spoken = num;
        turn += 1;
    }

    println!("Starting turn: {}", turn);

    while turn <= 30000000 {
        let output_num = match last_spoken_turn {
            Some(last_turn) => turn - last_turn - 1,
            None => 0
        };

        //println!("Turn {}: last {}, current {}, was first: {:?}", turn, last_spoken, output_num, last_spoken_turn);

        last_spoken_turn = last_spoken_turns.insert(output_num, turn);

        // Iterate
        last_spoken = output_num;
        turn += 1;
    }

    println!("{}", last_spoken);
}
