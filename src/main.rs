/// Converts a heading into an absolute figure.
///
/// e.g.
/// 0 degrees = 0 degrees
/// 90 degrees = 90 degrees
/// 361 degrees = 1 degree
/// -1 degrees = 359 degrees
fn absolute_heading(mut heading: i32) -> i32 {
    while heading < 0 {
        heading += 360;
    }

    heading % 360
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum ActionType {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

#[derive(Debug)]
struct Action {
    kind: ActionType,
    value: i32
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (action, distance) = line.split_at(1);

            let kind = match action {
                "N" => ActionType::North,
                "S" => ActionType::South,
                "E" => ActionType::East,
                "W" => ActionType::West,
                "L" => ActionType::Left,
                "R" => ActionType::Right,
                "F" => ActionType::Forward,
                x => panic!("Invalid action: {:?}", x)
            };

            Action {
                kind,
                value: distance.parse().expect("Failed to parse distance")
            }
        })
        .collect::<Vec<_>>();

    let (mut x, mut y) = (0i32, 0i32);
    let (mut way_x, mut way_y) = (10i32, -1i32);

    for action in input {
        println!("Executing action: {:?}", action);

        match action.kind {
            ActionType::North => way_y -= action.value,
            ActionType::South => way_y += action.value,
            ActionType::East => way_x += action.value,
            ActionType::West => way_x -= action.value,
            ActionType::Left | ActionType::Right => {
                let mut degrees = if action.kind == ActionType::Left {
                    absolute_heading(-action.value)
                } else {
                    action.value
                };

                while degrees > 0 {
                    let old_x = way_x;
                    let old_y = way_y;

                    way_y = old_x;
                    way_x = -old_y;

                    degrees -= 90;
                }
            },
            ActionType::Forward => {
                x += way_x * action.value;
                y += way_y * action.value;
            }
        }

        println!("New location: {} x {}", x, y);
        println!("Way: {} x {}", way_x, way_y);
    }

    println!("Distance: {}", x.abs() + y.abs());
}
