/// Converts the ships heading into a distance "delta".
fn heading_to_delta(heading: i32) -> (i32, i32) {
    match heading {
        0 => (0, -1),
        90 => (1, 0),
        180 => (0, 1),
        270 => (-1, 0),
        x => panic!("Bad heading: {}", x)
    }
}

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

#[derive(Debug)]
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

    let mut heading = 90;
    let (mut x, mut y) = (0i32, 0i32);

    for action in input {
        println!("Executing action: {:?}", action);

        match action.kind {
            ActionType::North => y -= action.value,
            ActionType::South => y += action.value,
            ActionType::East => x += action.value,
            ActionType::West => x -= action.value,
            ActionType::Left => heading = absolute_heading(heading - action.value),
            ActionType::Right => heading = absolute_heading(heading + action.value),
            ActionType::Forward => {
                let (x_delta, y_delta) = heading_to_delta(heading);
                x += x_delta * action.value;
                y += y_delta * action.value;
            }
        }

        println!("New location: {} x {}", x, y);
    }

    println!("Distance: {}", x.abs() + y.abs());
}
