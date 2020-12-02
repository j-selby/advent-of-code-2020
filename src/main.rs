use regex::Regex;

fn main() {
    let mut input = std::fs::read_to_string("input").expect("Failed to read input");

    let re = Regex::new(r"^(?P<from>[0-9]+)-(?P<to>[0-9]+) (?P<policy>\w): (?P<password>.*)$").unwrap();

    let result = input.lines()
        .map(|line| {
            // Parse line
            let captures = re.captures(line).expect("Failed to parse password");
            let start : usize = captures.name("from").expect("No from range").as_str().parse()
                .expect("Failed to parse starting number");
            let end : usize = captures.name("to").expect("No to range").as_str().parse()
                .expect("Failed to parse ending number");
            let policy = captures.name("policy").expect("No match letter").as_str();
            let password = captures.name("password").expect("No password").as_str();

            let first = password.chars().nth(start - 1) == policy.chars().next();
            let second = password.chars().nth(end - 1) ==  policy.chars().next();

            (first && !second) || (second && !first)
        })
        .filter(|x| *x)
        .count();

    println!("input: {:?}", result);
}
