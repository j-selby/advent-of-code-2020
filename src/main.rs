#![feature(str_split_once)]

#[derive(Clone)]
enum InstructionValue {
    Acc { value: i64 },
    Jmp { relative_value: i64 },
    Nop { relative_value: i64 },
}

fn parse_instr_integer(mut value: &str) -> i64 {
    if value.starts_with("+") {
        value = value.strip_prefix("+").expect("Failed to strip + prefix");
    }

    value.parse().expect("Failed to parse instr integer")
}

#[derive(Clone)]
struct Instruction {
    value: InstructionValue,
    visited: bool,
}

enum StateMachineState {
    SuccessfulInstruction,
    Corrupt,
    EndOfFile
}

struct StateMachine {
    accumulator: i64,
    pc: usize
}

impl StateMachine {
    fn run(&mut self, instrs: &mut Vec<Instruction>) -> StateMachineState {
        let instruction = instrs
            .get_mut(self.pc)
            .expect("Failed to find instruction for PC");

        if instruction.visited {
            println!("Hit instruction {} that was already visited!", self.pc);
            return StateMachineState::Corrupt;
        }

        instruction.visited = true;

        match &instruction.value {
            InstructionValue::Acc { value } => {
                self.accumulator += value;
                self.pc += 1;
            }
            InstructionValue::Jmp { relative_value } => {
                self.pc = (self.pc as i64 + relative_value) as usize;
            }
            InstructionValue::Nop { .. } => {
                self.pc += 1;
            }
        }

        if self.pc == instrs.len() {
            StateMachineState::EndOfFile
        } else {
            StateMachineState::SuccessfulInstruction
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    // Parse the instructions into an array
    let mut instrs = input
        .lines()
        .map(|line| {
            let (instr_name, instr_value) = line.split_once(" ").expect("Failed to split line");

            let value = match instr_name {
                "acc" => InstructionValue::Acc {
                    value: parse_instr_integer(instr_value),
                },
                "jmp" => InstructionValue::Jmp {
                    relative_value: parse_instr_integer(instr_value),
                },
                "nop" => InstructionValue::Nop {
                    relative_value: parse_instr_integer(instr_value),
                },
                x => panic!("Invalid instruction: {:?}", x),
            };

            Instruction {
                value,
                visited: false,
            }
        })
        .collect::<Vec<_>>();

    // Iterate through all jmps and nops to see which one completes execution
    let mut modify_pc = 0;

    let mut machine;

    'check_loop:
    loop {
        let mut instrs = instrs.clone();

        machine = StateMachine {
            accumulator: 0,
            pc: 0
        };

        // Modify an instruction
        loop {
            let instr = &mut instrs[modify_pc];

            modify_pc += 1;

            let new_value = match &instr.value {
                InstructionValue::Acc { .. } => continue,
                InstructionValue::Jmp { relative_value } => {
                    InstructionValue::Nop {
                        relative_value: *relative_value
                    }
                }
                InstructionValue::Nop { relative_value } => {
                    InstructionValue::Jmp {
                        relative_value: *relative_value
                    }

                }
            };

            instr.value = new_value;
            break;
        }

        // Run the virtual machine
        loop {
            match machine.run(&mut instrs) {
                StateMachineState::SuccessfulInstruction => continue,
                StateMachineState::Corrupt => break,
                StateMachineState::EndOfFile => break 'check_loop
            }
        }
    }

    println!("Acc at termination: {}", machine.accumulator);

}
