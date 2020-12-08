#![feature(str_split_once)]

enum InstructionValue {
    Acc{ value : i64 },
    Jmp{ relative_value : i64 },
    Nop
}

fn parse_instr_integer(mut value : &str) -> i64 {
    if value.starts_with("+") {
        value = value.strip_prefix("+")
            .expect("Failed to strip + prefix");
    }

    value.parse().expect("Failed to parse instr integer")
}

struct Instruction {
    value : InstructionValue,
    visited : bool,
}

struct StateMachine {
    accumulator: i64,
    pc: usize
}

impl StateMachine {
    fn run(&mut self, instruction: &mut Instruction) -> bool {
        if instruction.visited {
            println!("Hit instruction that was already visited!");
            return false
        }

        instruction.visited = true;

        println!("Visiting: {}", self.pc);

        match &instruction.value {
            InstructionValue::Acc { value } => {
                self.accumulator += value;
                self.pc += 1;
            }
            InstructionValue::Jmp { relative_value } => {
                self.pc = (self.pc as i64 + relative_value) as usize;
            }
            InstructionValue::Nop => {
                self.pc += 1;
            }
        }

        true
    }
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Failed to read input");

    let mut machine = StateMachine {
        accumulator: 0,
        pc: 0
    };

    // Parse the instructions into an array
    let mut instrs = input.lines()
        .map(|line| {
            let (instr_name, instr_value) = line.split_once(" ").expect("Failed to split line");

            let value = match instr_name {
                "acc" => InstructionValue::Acc { value: parse_instr_integer(instr_value) },
                "jmp" => InstructionValue::Jmp { relative_value: parse_instr_integer(instr_value) },
                "nop" => InstructionValue::Nop,
                x => panic!("Invalid instruction: {:?}", x)
            };

            Instruction {
                value,
                visited: false
            }
        })
        .collect::<Vec<_>>();

    loop {
        if !machine.run(instrs.get_mut(machine.pc).expect("Failed to find instruction for PC")) {
            break;
        }
    }

    println!("Acc at termination: {}", machine.accumulator);
}
