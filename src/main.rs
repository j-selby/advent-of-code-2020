
fn main() {
    let mut input = std::fs::read_to_string("input").expect("Failed to read input");

    let map: Vec<Vec<bool>> = input.lines().map(|x|
        x.trim().chars().map(|x| if x == '#' { true } else { false }).collect()
    ).collect();

    let pitches = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let pitch_results = pitches.iter().map(|pitch| {
        let (mut x, mut y) = (0, 0);
        let (x_pitch, y_pitch) = pitch;

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

        trees
    }).fold(1, |acc, next| acc * next);

    println!("{}", pitch_results);



}
