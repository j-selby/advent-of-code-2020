#![feature(str_split_once)]

use std::collections::HashMap;
use regex::{Regex, Captures};
use std::str::FromStr;
use std::fmt::Debug;

/// A passport rule, from passport metadata, verifies both a regular expression, and optionally,
/// an additional validator.
struct PassportRule<'key_duration, 'regex_duration> {
    key: &'key_duration str,
    regex: &'regex_duration Regex,
    validator: Option<Box<dyn Fn(Captures) -> bool>>,
}

impl<'key_duration, 'regex_duration> PassportRule<'key_duration, 'regex_duration> {
    fn new(key: &'key_duration str, regex: &'regex_duration Regex, validator: Option<Box<dyn Fn(Captures) -> bool>>) -> Self {
        PassportRule {
            key,
            regex,
            validator,
        }
    }

    // Evaluates
    fn evaluate(&self, map: &HashMap<&str, &str>) -> bool {
        // Make sure the map has an entry
        match map.get(self.key) {
            Some(value) => {
                // Make sure the regex matches
                match self.regex.captures(value) {
                    Some(entries) => {
                        // If there is a custom validator, check that, else pass the rule
                        match &self.validator {
                            &Some(ref validator) => validator(entries),
                            _ => true
                        }
                    }
                    _ => false
                }
            }
            _ => false
        }
    }
}

// Creates a passport rule with fixed logic which validates that a integer capture group from the regex
// matches a range.
fn new_range_passport_rule<'key_length, 'regex_length, RangeKind: 'static + Ord + FromStr>(key: &'key_length str, regex: &'regex_length Regex, start: RangeKind, end: RangeKind) -> PassportRule<'key_length, 'regex_length>
    where <RangeKind as FromStr>::Err: Debug {
    PassportRule::new(key, regex, Some(Box::new(
        move |captures: Captures| {
            let value = captures.get(1).expect("Failed to get year component")
                .as_str().parse::<RangeKind>().expect("Failed to parse year");
            value >= start && value <= end
        }
    )))
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    // Rule regex
    let year_regex = Regex::new(r"^([1-9][0-9]{3})$").expect("Bad year regex");
    let height_regex = Regex::new(r"^([0-9]+)(in|cm)$").expect("Bad height regex");
    let hair_colour_regex = Regex::new(r"^#([0-9a-fA-F]{6})$").expect("Bad hair colour regex");
    let eye_colour_regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").expect("Bad height regex");
    let passport_id_regex = Regex::new(r"^([0-9]{9})$").expect("Bad passport ID");

    // The actual, executable rules
    let rules = [
        new_range_passport_rule("byr", &year_regex, 1920, 2002),
        new_range_passport_rule("iyr", &year_regex, 2010, 2020),
        new_range_passport_rule("eyr", &year_regex, 2020, 2030),
        PassportRule::new("hgt", &height_regex, Some(Box::new(
            |height_captures: Captures| {
                let value = height_captures.get(1).expect("Failed to get height value")
                    .as_str().parse::<i32>().expect("Failed to parse height");
                let suffix = height_captures.get(2).expect("Failed to get height unit").as_str();

                let (min, max) = if suffix == "cm" {
                    (150, 193)
                } else {
                    (59, 76)
                };

                value >= min && value <= max
            }
        ))),
        PassportRule::new("hcl", &hair_colour_regex, None),
        PassportRule::new("ecl", &eye_colour_regex, None),
        PassportRule::new("pid", &passport_id_regex, None)
    ];

    // Convert input into dictionaries, split by blank lines
    let valid_passports = input.split("\n\n").map(|entry|
        entry.split_whitespace()
            .filter(|entry| !entry.is_empty())
            .map(|pair| pair.split_once(":").expect("Invalid key-value pair"))
            .collect::<HashMap<_, _>>()
    ).filter(|passport| {
        // Evaluate each passport rule and only return if all rules match
        let mut all_validate = true;
        for rule in &rules {
            if !rule.evaluate(passport) {
                all_validate = false;
                break;
            }
        }

        all_validate
    }).count();

    println!("Valid passports: {}", valid_passports);
}
