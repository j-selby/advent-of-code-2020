#![feature(str_split_once)]

use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Assignment { index: i64, value: i64 },
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    let input: Vec<Instruction> = input
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(|line| {
            let (assign, value) = line.split_once(" = ").expect("Failed to find separator");

            if assign == "mask" {
                Instruction::Mask(value.chars().rev().collect::<String>())
            } else {
                let index = assign
                    .strip_prefix("mem[")
                    .expect("Not a mem instruction?")
                    .strip_suffix("]")
                    .expect("Not a mem instruction (missing end)?")
                    .parse::<i64>()
                    .expect("Failed to parse index");

                Instruction::Assignment {
                    index,
                    value: value.parse().expect("Failed to parse value"),
                }
            }
        })
        .collect();

    let mut current_mask = String::new();

    let mut memory = HashMap::new();

    for instr in input {
        match instr {
            Instruction::Mask(mask) => current_mask = mask,
            Instruction::Assignment { index, mut value } => {
                // Mask bits
                for (offset, char) in current_mask.chars().enumerate() {
                    match char {
                        'X' => continue,
                        '1' => value |= (1 << offset),
                        '0' => value &= !(1 << offset),
                        x => panic!("Bad char: {}", x),
                    }
                }

                memory.insert(index, value);
            }
        }
    }


    println!("{:?}", memory.values().sum::<i64>());
}
