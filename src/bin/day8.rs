use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
enum Operation {
    Acc,
    Nop,
    Jmp,
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    value: i32,
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let op_and_value: Vec<&str> = line.split(" ").collect();
        let operation = match op_and_value[0] {
            "nop" => Operation::Nop,
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            _ => panic!("Not possible"),
        };
        let value = op_and_value[1].parse::<i32>().unwrap();
        Self { operation, value }
    }
}

#[derive(Debug)]
struct FindResult {
    value: i32,
    is_cycle: bool,
}

fn find_cycle_or_finish(instructions: &Vec<Instruction>) -> FindResult {
    let mut seen_instruction_idx = HashSet::<usize>::new();
    let mut current_id = 0;
    let mut global_value = 0;
    loop {
        if current_id == instructions.len() {
            return FindResult {
                value: global_value,
                is_cycle: false,
            };
        }
        if seen_instruction_idx.contains(&current_id) {
            return FindResult {
                value: global_value,
                is_cycle: true,
            };
        }
        seen_instruction_idx.insert(current_id);
        let current_instruction = &instructions[current_id];
        // println!("{:?} {} {}", current_instruction, current_id, global_value);
        match current_instruction.operation {
            Operation::Acc => {
                global_value += current_instruction.value;
                current_id += 1;
            }
            Operation::Nop => {
                current_id += 1;
            }
            Operation::Jmp => {
                let jumped_id = (current_id as i32) + current_instruction.value;
                current_id = jumped_id as usize;
            }
        }
    }
}

fn swap_instruction_operation(instruction: &mut Instruction) {
    match instruction.operation {
        Operation::Nop => {
            instruction.operation = Operation::Jmp;
        }
        Operation::Jmp => {
            instruction.operation = Operation::Nop;
        }
        Operation::Acc => {}
    }
}

fn main() {
    let content = fs::read_to_string("inputs/day8.txt").expect("Expected a file");
    let mut instructions: Vec<Instruction> = content.lines().map(Instruction::parse).collect();

    let part1 = find_cycle_or_finish(&instructions);
    println!("Part 1: {}", part1.value);

    for index in 0..instructions.len() {
        swap_instruction_operation(&mut instructions[index]);
        let part2 = find_cycle_or_finish(&instructions);
        if !part2.is_cycle {
            println!("Part 2: {}", part2.value);
            break;
        }
        swap_instruction_operation(&mut instructions[index]);
    }
}
