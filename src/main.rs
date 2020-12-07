#![feature(str_split_once)]

use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

/// Parses a bag title, returning the color components
fn parse_bag_name(input: &str) -> (&str, &str) {
    if !input.ends_with("bag") && !input.ends_with("bags") {
        panic!(
            "Attempt to parse bag name that isn't a \"bag\": {:?}",
            input
        );
    }

    input
        .split_whitespace()
        .take(2)
        .collect_tuple()
        .expect("Not enough elements to parse bag name")
}

// A bag quantity describes the capacity of a bag to hold another bag, and how many
// it can hold.
#[derive(Debug, Clone)]
struct BagQuantity<'a> {
    name: (&'a str, &'a str),
    qtr: usize,
}

impl<'a> BagQuantity<'a> {
    fn new_from_input(input: &'a str) -> BagQuantity<'a> {
        let (number, components) = input
            .split_once(" ")
            .expect("Failed to split up bag with count");

        BagQuantity {
            name: parse_bag_name(components),
            qtr: number.parse().expect("Failed to parse bag quantity"),
        }
    }
}

// A bag is something which can either be contained, or contain other bags.
#[derive(Debug)]
struct Bag<'a> {
    name: (&'a str, &'a str),
    subtypes: Option<Vec<BagQuantity<'a>>>,
    parents: Option<Vec<BagQuantity<'a>>>,
}

// Counts the number of bags iteratively for the current bag count
fn iterate_bag(all_bags: &HashMap<(&str, &str), Bag>, current_bag: &(&str, &str)) -> usize {
    let bag = all_bags.get(current_bag).expect("Failed to get sub bag");
    let mut count : usize = 0;

    if let Some(sub_bags) = bag.subtypes.as_ref() {
        for sub_bag in sub_bags {
            count += sub_bag.qtr;
            count += sub_bag.qtr * iterate_bag(all_bags, &sub_bag.name);
        }
    }

    count
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    // Parse bags into the "Bag" structure
    let mut bags = input
        .lines()
        .map(|line| {
            // Break apart the string
            let (bag_type, subtypes) = line
                .split_once(" contain ")
                .expect("Unable to find \"contain\" keyword");
            let subtypes = subtypes
                .strip_suffix(".")
                .expect("Failed to strip end full stop.");

            // Parse them into bag names
            let bag_type = parse_bag_name(bag_type);

            let subtypes = if subtypes == "no other bags" {
                None
            } else {
                Some(
                    subtypes
                        .split(", ")
                        .map(BagQuantity::new_from_input)
                        .collect::<Vec<_>>(),
                )
            };

            let bag = Bag {
                name: bag_type,
                subtypes,
                parents: None,
            };

            (bag_type, bag)
        })
        .collect::<HashMap<_, _>>();

    // Build a list of keys, removing links to the parent structure
    let keys = bags
        .keys()
        .map(|(left, right)| (left.to_string(), right.to_string()))
        .collect_vec();

    // Fill in the parents structure in reverse
    for (left, right) in &keys {
        let parent_bag = bags
            .get(&(left.as_str(), right.as_str()))
            .expect("Failed to find element matching key");
        let parent_name = parent_bag.name;

        if let Some(bag_elements) = parent_bag.subtypes.clone() {
            for bag_element in bag_elements {
                let child_bag = bags
                    .get_mut(&bag_element.name)
                    .expect("Failed to find child element");

                if child_bag.parents.is_none() {
                    child_bag.parents = Some(Vec::with_capacity(1));
                }

                let child_bag_parents = child_bag
                    .parents
                    .as_mut()
                    .expect("Failed to get child parents");
                child_bag_parents.push(BagQuantity {
                    name: parent_name,
                    qtr: bag_element.qtr,
                });
            }
        }
    }

    let bag_count = iterate_bag(&bags, &("shiny", "gold"));

    println!("Bags: {}", bag_count);
}
