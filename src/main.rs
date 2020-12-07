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
    qtr: i32,
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

    // Next, find our shiny gold bag
    let root = &bags[&("shiny", "gold")];

    let mut explored = HashSet::new();
    let mut to_explore = root.parents.as_ref().expect("Bag should contain parents")
        .to_owned();

    let mut bag_count = 0;

    // Crawl the tree iteratively
    while let Some(next_bag) = to_explore.pop() {
        if !explored.insert(next_bag.name) {
            continue;
        }

        bag_count += 1;

        // Lookup the next bag
        let parent_bag = bags
            .get(&next_bag.name)
            .expect("Failed to find parent of next bag");

        if let Some(parent_bags) = parent_bag.parents.as_ref() {
            to_explore.extend(parent_bags.to_owned());
        }
    }

    println!("Bags: {}", bag_count);
}
