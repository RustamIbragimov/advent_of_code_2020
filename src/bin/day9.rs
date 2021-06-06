use std::fs;

fn is_valid(numbers: &Vec<i64>, index: usize, lookback: usize) -> bool {
    if (index as i32) - (lookback as i32) < 0 {
        return true;
    }
    let target = numbers[index];
    let (start_index, end_index) = (index - lookback, index);
    for i in start_index..end_index {
        for j in i + 1..end_index {
            if numbers[i] == numbers[j] {
                continue;
            }
            if numbers[i] + numbers[j] == target {
                return true;
            }
        }
    }
    false
}

#[derive(Debug)]
struct Boundary {
    left: usize,
    right: usize,
}

fn find_contiguous_set(numbers: &Vec<i64>, target: i64) -> Boundary {
    let (mut left, mut right) = (0, 0);
    let mut sum = 0;
    while right < numbers.len() {
        sum += numbers[right];
        if sum == target {
            break;
        } else if sum > target {
            while left < right && sum > target {
                sum -= numbers[left];
                left += 1;
            }
            if sum == target {
                break;
            }
        }
        right += 1;
    }
    Boundary { left, right }
}

fn main() {
    let content = fs::read_to_string("inputs/day9.txt").expect("Expected a file");
    let numbers: Vec<i64> = content
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut invalid_number = 0;
    for index in 0..numbers.len() {
        if !is_valid(&numbers, index, 25) {
            invalid_number = numbers[index];
            break;
        }
    }
    println!("Part 1: {}", invalid_number);

    let boundary = find_contiguous_set(&numbers, invalid_number);
    let contiguous_slice = &numbers[boundary.left..boundary.right + 1];
    let (max, min) = (
        contiguous_slice.iter().max().unwrap(),
        contiguous_slice.iter().min().unwrap(),
    );
    println!("Part 2: {}", max + min);
}
