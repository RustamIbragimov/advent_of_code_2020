use std::fs;

#[derive(Debug)]
struct Policy<'a> {
    first_num: i32,
    second_num: i32,
    letter: char,
    password: &'a str
}

impl<'a> Policy<'a> {
    fn parse(line: &'a str) -> Self {
        let split: Vec<_> = line.split(":").map(|s| s.trim()).collect();
        let (pattern, password) = (split[0], split[1]);
        let split: Vec<_> = pattern.split(" ").collect();
        let (numbers, letter) = (split[0], split[1]);
        let split: Vec<i32> = numbers.split("-").map(|s| s.parse().unwrap()).collect();
        let (first_num, second_num) = (split[0], split[1]);

        Self {
            first_num,
            second_num,
            letter: letter.chars().next().unwrap(),
            password
        }
    }

    fn is_valid_part_1(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.letter {
                count += 1;
            }
        }
        count >= self.first_num && count <= self.second_num
    }

    fn is_valid_part_2(&self) -> bool {
        let chars: Vec<_> = self.password.chars().collect();
        let mut found_times = 0;
        if chars[(self.first_num - 1) as usize] == self.letter {
            found_times += 1;
        }
        if chars[(self.second_num - 1) as usize] == self.letter {
            found_times += 1;
        }
        found_times == 1
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/day2.txt").expect("Expected a file");
    let policies: Vec<_> = contents
        .lines()
        .map(|s| Policy::parse(s))
        .collect();

    let (mut valid_part_1_count, mut valid_part_2_count) = (0, 0);
    for policy in policies {
        if policy.is_valid_part_1() {
            valid_part_1_count += 1;
        }
        if policy.is_valid_part_2() {
            valid_part_2_count += 1;
        }
    }
    println!("Part 1: number of valid passwords {}", valid_part_1_count);
    println!("Part 2: number of valid passwords {}", valid_part_2_count);
}