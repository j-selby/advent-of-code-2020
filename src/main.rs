fn main() {
    let mut input = std::fs::read_to_string("input").expect("Failed to read input");
    let numbers: Vec<i32> = input.split_whitespace().map(|num| num.parse().expect("Failed to parse number")).collect();

    for num in &numbers {
        for other_num in &numbers {
            if num + other_num == 2020 {
                println!("{} * {} = {}", num, other_num, num * other_num);
            }
        }
    }

    println!("input: {:?}", numbers);
}
