#![feature(str_split_once)]

#[derive(Debug)]
struct Rule<'a> {
    name: &'a str,
    ranges: Vec<(i64, i64)>,
}

#[derive(Debug)]
struct Ticket {
    numbers: Vec<i64>,
}

fn parse_tickets(input: &str) -> Vec<Ticket> {
    input
        .lines()
        .skip(1)
        .map(|line| Ticket {
            numbers: line
                .split(",")
                .map(|line_value| {
                    line_value
                        .parse()
                        .expect("Failed to parse ticket line value")
                })
                .collect(),
        })
        .collect()
}

/// Evaluates if a value matches a rule
fn evaluate_rule(value: i64, rule: &Rule) -> bool {
    for (low, high) in &rule.ranges {
        if value >= *low && value <= *high {
            return true;
        }
    }

    false
}

/// Validates that a ticket can have any possible match for any rule
fn ticket_error_rate(ticket: &Ticket, rules: &[Rule]) -> i64 {
    // Check if any numbers *don't* have a matching rule and invert
    ticket
        .numbers
        .iter()
        .filter(|num| {
            rules
                .iter()
                .filter(|rule| evaluate_rule(**num, rule))
                .count()
                == 0
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    // 3 sections: rules, your ticket, nearby tickets. split this:
    let mut input_splits = input.split("\n\n");

    let rules = input_splits.next().expect("Failed to find rules section");
    let your_ticket = input_splits.next().expect("Failed to find your ticket");
    let nearby_tickets = input_splits.next().expect("Failed to find nearby tickets");

    // First, parse the rules
    let rules = rules
        .lines()
        .map(|line| {
            let (name, matches) = line.split_once(": ").expect("Failed to find delimiter");
            let ranges = matches
                .split(" or ")
                .map(|range| {
                    let (low, high) = range
                        .split_once("-")
                        .expect("Failed to find range delimiter");
                    (
                        low.parse().expect("Failed to parse low"),
                        high.parse().expect("Failed to parse high"),
                    )
                })
                .collect();

            Rule { name, ranges }
        })
        .collect::<Vec<_>>();

    // and the tickets:
    let your_ticket = parse_tickets(your_ticket)
        .pop()
        .expect("Failed to find ticket");
    let nearby_tickets = parse_tickets(nearby_tickets);

    let bad_tickets : i64 = nearby_tickets
        .iter()
        .map(|ticket| ticket_error_rate(ticket, &rules))
        .sum();

    println!("{:#?}", bad_tickets);
}
