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

    let direction_options = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1)
    ];

    // Run iterations
    let mut count = 0;
    loop {
        // Take a copy for stagnation checks
        let last_input = input.clone();

        for (y, row) in last_input.iter().enumerate() {
            for (x, seat) in row.iter().enumerate() {
                // Get adjacent seats
                let mut adjacent_seats = 0;


                for (direction_x, direction_y) in &direction_options {
                    let mut check_x = x as i32;
                    let mut check_y = y as i32;

                    // Wait until we find a seat we can see
                    'main_loop:
                    loop {
                        check_x += direction_x;
                        check_y += direction_y;

                        if check_y >= 0
                            && check_x >= 0
                            && check_y < last_input.len() as i32
                            && check_x < row.len() as i32
                            //&& !(check_x == x_abs && check_y == y_abs)
                        {
                            match last_input[check_y as usize][check_x as usize] {
                                SeatType::Empty => break 'main_loop,
                                SeatType::Occupied => {
                                    adjacent_seats += 1;
                                    break 'main_loop;
                                }
                                SeatType::Floor => continue 'main_loop,
                            }
                        } else {
                            break 'main_loop;
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
                        if adjacent_seats >= 5 {
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
