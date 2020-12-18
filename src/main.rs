#![feature(str_split_once)]

use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Assignment { index: i64, value: i64 },
}

fn emit_address(mut address: i64, mask : &String, mask_offset: usize, addresses : &mut Vec<i64>) {
    for (offset, char) in mask.chars().enumerate().skip(mask_offset) {
        match char {
            'X' => {
                // Fork with this bit set to 1:
                emit_address(address | (1 << offset), mask, offset + 1, addresses);

                // Emit with this bit set to 0:
                address &= !(1 << offset);
            },
            '1' => address |= (1 << offset),
            '0' => { /* no-op */ },
            x => panic!("Bad char: {}", x),
        }
    }

    addresses.push(address);
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
    let mut addresses = Vec::new();

    for instr in input {
        addresses.clear();

        match instr {
            Instruction::Mask(mask) => current_mask = mask,
            Instruction::Assignment { index, mut value } => {
                emit_address(index, &current_mask, 0, &mut addresses);

                for address in &addresses {
                    memory.insert(*address, value);
                }
            }
        }
    }

    println!("{:?}", memory.values().sum::<i64>());
}
