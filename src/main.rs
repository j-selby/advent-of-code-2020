use regex::Regex;

fn main() {
    let mut input = std::fs::read_to_string("input").expect("Failed to read input");

    let map: Vec<Vec<bool>> = input.lines().map(|x|
        x.trim().chars().map(|x| if x == '#' { true } else { false }).collect()
    ).collect();

    let (mut x, mut y) = (0, 0);
    let (x_pitch, y_pitch) = (3, 1);

    let mut trees = 0;
    while y < map.len() {
        let line = &map[y];
        if line[x % line.len()] {
            trees += 1;
        }

        x += x_pitch;
        y += y_pitch;
    }

    println!("Trees: {}", trees);
}
