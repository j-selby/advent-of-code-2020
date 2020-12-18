fn main() {
    let input = std::fs::read_to_string("input")
        .expect("Failed to read input");

    let mut input = input.lines();

    let earliest_time = input
        .next()
        .expect("Failed to find earliest time")
        .parse::<i32>()
        .expect("Failed to parse earliest time");

    let bus_times: Vec<i32> = input
        .next()
        .expect("Failed to find bus IDs")
        .split(",")
        .filter(|x| x != &"x")
        .map(|x| x.parse().expect("Failed to parse bus ID"))
        .collect();

    let mut current_time = earliest_time;
    let matching_id;

    loop {
        match bus_times.iter().filter(|x| current_time % **x == 0).next() {
            None => {},
            Some(match_id) => {
                matching_id = *match_id;
                break
            }
        }

        current_time += 1;
    }

    println!("Matching ID: {} @ {}", matching_id, current_time);
    println!("Wait time: {}", matching_id * (current_time - earliest_time));
}
