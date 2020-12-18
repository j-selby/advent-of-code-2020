fn modulo(mut x : i64, m : i64) -> i64 {
    while x < 0 {
        x += m;
    }

    x % m
}

fn main() {
    let input = std::fs::read_to_string("input")
        .expect("Failed to read input");

    let mut input = input.lines();

    let earliest_time = input
        .next()
        .expect("Failed to find earliest time")
        .parse::<u64>()
        .expect("Failed to parse earliest time");

    let bus_times: Vec<Option<u64>> = input
        .next()
        .expect("Failed to find bus IDs")
        .split(",")
        .map(|x| {
            if x == "x" {
                None
            } else {
                Some(x.parse().expect("Failed to parse bus ID"))
            }
        })
        .collect();

    let bus_times_n_offset : Vec<(usize, u64)> = bus_times.iter().enumerate()
        .filter(|(_x, delta)| delta.is_some())
        .map(|(x, delta)| (x, delta.unwrap()))
        .collect();

    let u : Vec<i64> = bus_times_n_offset.iter()
        .map(|(x, delta)| modulo(-(*x as i64), *delta as i64))
        .collect();

    let s : Vec<i64> = bus_times_n_offset.iter()
        .map(|(_x, delta)| *delta as i64)
        .collect();

    // Fuck number theory:
    let solution = ring_algorithm::chinese_remainder_theorem(&u, &s);

    // 604510652639806
    println!("{:?}", solution);
}
