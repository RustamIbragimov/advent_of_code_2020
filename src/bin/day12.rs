use std::fs;

#[derive(Debug)]
enum Instruction {
    North { value: i32 },
    South { value: i32 },
    East { value: i32 },
    West { value: i32 },
    Left { degree: i32 },
    Right { degree: i32 },
    Forward { value: i32 },
}

fn line_to_instruction(line: &str) -> Instruction { 
    let chars: Vec<char> = line.chars().collect();
    let action = chars[0];
    let value: i32 = chars[1..].iter().collect::<String>().parse().unwrap();

    match action {
        'N' => Instruction::North { value },
        'S' => Instruction::South { value },
        'E' => Instruction::East { value },
        'W' => Instruction::West { value },
        'L' => Instruction::Left { degree:value },
        'R' => Instruction::Right { degree:value },
        'F' => Instruction::Forward { value },
        _ => panic!("Invalid action")
    }
}

fn main() {
    let content = fs::read_to_string("inputs/day12.txt").expect("Expected a file");
    let instructions: Vec<Instruction> = content
        .lines()
        .map(|line| line_to_instruction(line))
        .collect();
    
    let (mut x, mut y, mut facing_degree) = (0, 0, 90);
    for instruction in &instructions {
        match instruction {
            Instruction::North { value } => y += value,
            Instruction::South { value } => y -= value,
            Instruction::West { value } => x -= value,
            Instruction::East { value } => x += value,
            Instruction::Right { degree } => facing_degree = (facing_degree + degree + 360) % 360,
            Instruction::Left { degree } => facing_degree = (facing_degree - degree + 360) % 360,
            Instruction::Forward { value } => {
                match facing_degree {
                    0 => y += value,
                    90 => x += value,
                    180 => y -= value,
                    270 => x -= value,
                    _ => panic!("Invalid facing degree {}", facing_degree),
                }
            }
        }
    }

    println!("Part 1: x={}, y={}, Manhatan distance={}", x, y, x.abs() + y.abs());

    // TODO (PART 2)
}