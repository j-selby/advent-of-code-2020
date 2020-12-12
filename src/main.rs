#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
enum SeatType {
    Empty,
    Occupied,
    Floor,
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    let mut input = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    'L' => SeatType::Empty,
                    '#' => SeatType::Occupied,
                    '.' => SeatType::Floor,
                    x => panic!("Invalid char: {:?}", x),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Run iterations
    let mut count = 0;
    loop {
        // Take a copy for stagnation checks
        let last_input = input.clone();

        for (y, row) in last_input.iter().enumerate() {
            for (x, seat) in row.iter().enumerate() {
                // Get adjacent seats
                let mut adjacent_seats = 0;

                let y_abs = y as i32;
                let x_abs = x as i32;

                for check_y in y_abs - 1..=y_abs + 1 {
                    for check_x in x_abs - 1..=x_abs + 1 {
                        if check_y >= 0
                            && check_x >= 0
                            && check_y < last_input.len() as i32
                            && check_x < row.len() as i32
                            && !(check_x == x_abs && check_y == y_abs)
                        {
                            if last_input[check_y as usize][check_x as usize] == SeatType::Occupied
                            {
                                adjacent_seats += 1;
                            }
                        }
                    }
                }

                match seat {
                    SeatType::Empty => {
                        if adjacent_seats == 0 {
                            input[y][x] = SeatType::Occupied;
                        }
                    }
                    SeatType::Occupied => {
                        if adjacent_seats >= 4 {
                            input[y][x] = SeatType::Empty;
                        }
                    }
                    SeatType::Floor => {
                        // No-op
                    }
                }
            }
        }

        count += 1;

        println!("Completed iter: {}", count);

        // Print state
        for row in input.iter() {
            for seat in row.iter() {
                let char = match seat {
                    SeatType::Empty => 'L',
                    SeatType::Occupied => '#',
                    SeatType::Floor => '.'
                };
                print!("{}", char);
            }
            println!();
        }

        if input == last_input {
            println!("Completed after {} counts", count);
            break;
        }
    }

    let occupied_seats : usize = input
        .iter()
        .map(|x| x.iter().filter(|x| **x == SeatType::Occupied).count())
        .sum();

    println!("Seats occupied: {}", occupied_seats);
}
