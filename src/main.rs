#![feature(str_split_once)]

#[derive(Debug)]
struct Seat {
    row: u32,
    column: u32
}

impl Seat {
    fn get_seat_id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    let result = input.lines()
        .map(|line| {
            let (mut row_min, mut row_max) = (0, 127);
            let (mut column_min, mut column_max) = (0, 7);

            let row_div = line.chars().take(7);
            let col_div = line.chars().skip(7).take(3);

            for row_option in row_div {
                let diff = (row_max - row_min) + 1;
                match row_option {
                    'F' => {
                        row_max -= diff / 2;
                    }
                    'B' => {
                        row_min += diff / 2;
                    }
                    x => panic!("Bad row char: {:?}", x)
                }
            }

            for col_option in col_div {
                let diff = (column_max - column_min) + 1;
                match col_option {
                    'L' => {
                        column_max -= diff / 2;
                    }
                    'R' => {
                        column_min += diff / 2;
                    }
                    x => panic!("Bad col char: {:?}", x)
                }
            }


            let seat =
                Seat {
                    row: row_min,
                    column: column_min
                };

            println!("{:?} = {:?}", line, seat);

            seat
        }).map(|x| x.get_seat_id()).max(); //.collect::<Vec<_>>();

    println!("Result: {:?}", result);
}
